# Chapter 6 Working with Object Storage

In our journey to build a robust and scalable storage engine, we've arrived at a crucial point: interfacing with object storage. Unlike traditional disk-based systems, our engine wants to decouple compute and storage by using cloud object stores like Amazon S3, Google Cloud Storage, or Azure Blob Storage. This architectural choice brings unique challenges and opportunities that we'll explore in this chapter.

At a high level, our storage engine interacts with object storage for three main things
1. storing and reading SSTable files
2. storing and reading manifest file
3. storing and reading WAL files

The following features of the object store are required
1. put
2. get
3. list
4. conditional put: atomic write (required for consistency)

### Using object_store

These features are abstracted away nicely by the `object_store` Rust crate so we will use that. The object_store crate provides a unified interface for interacting with various object storage systems, including cloud services and local filesystems. It offers the `ObjectStore` trait, which encapsulates the core capabilities our storage engine needs. By the way, this crate was written as part of the wonderful `datafusion` project and also maintained by them. It is a commonly-used crate in the open source Rust community at the moment.

```bash
cargo add object_store
```

What apis of `object_store` we need?

```rust
fn object_store_usage() {
    // listing manifest files and read the latest one
    let files_stream = object_store.list(Some(manifest_path));
    object_store.get(&manifest_file_path);

    // conditional put for atomic write manifest
    object_store.put_if_not_exists(manifest_path, data).await;
}
```

The trait we need
```rust
use futures::stream::BoxStream;
use object_store::path::Path;
use object_store::{
    PutResult, GetResult, ObjectMeta,
};

pub trait ObjectStore {
    async fn put_if_not_exists(path: &Path, data: Bytes) -> Result<PutResult, Error>;
    async fn get(path: &Path) -> Result<GetResult, Error>;
    /// why is list not async?
    fn list(path: &Path) -> BoxStream<'_, Result<ObjectMeta, Error>>;
}
```

why do we need our own extra layer of indirection here instead of calling to object_store directly?

## Considerations when working with object store

Working with object store means making network calls and these can fail, takes longer than accessing local disk. Hence, there are some necessary considerations when working with it. 

### Handling object storage limits

Different object storage systems have varying capabilities. For example, our previous design relied on object versioning for Write-Ahead Log (WAL) SST Compare-And-Swap (CAS) operations. However, this approach was abandoned due to limitations in services like S3 Express One Zone. Instead, we now use a combination of conditional puts and careful manifest management to achieve the same level of consistency without relying on object versioning.

### Performance

Interacting with object store has more latency than with local disk. Other techniques are used in other places to mitigate the latency impact
1. batch operations where possible
2. cache manifest file in memory and refresh it when necessary

### Handling failures

The crate `object_store` has [RetryConfig](https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html), requests will be retried up to some limit, using exponential backoff with jitter in [BackoffConfig](https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html).

### Ensure consistency with FenceableManifest

To handle concurrent writers and compactors, we implement the `FenceableManifest`.
This structure uses epoch numbers to detect and prevent conflicting writes, ensuring that only one writer or compactor can modify the manifest at a time.

```rust
/// pub(crate) is this should be used internally for the engine, not by client
pub(crate) struct FenceableManifest {
    stored_manifest: StoredManifest,
    local_epoch: u64,
    stored_epoch: Box<dyn Fn(&Manifest) -> u64 + Send>,
}
```


