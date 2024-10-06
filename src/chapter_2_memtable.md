# Chapter 2 Memtable

A memtable is where read and write requests hit at the beginning.

A memtable needs to support get and put key-value pairs because we need to write a key-value database after all.
```rust
impl Memtable {
    pub fn get(&self, key: &[u8]) -> Option<Bytes> {
        todo!()
    }
    
    pub fn put(&self, key: &[u8], value: &[u8]) {
        todo!()
    }
}
```

### Memtable implementation and data structures

why organize memtable as a tree?

Memtable implementation varies from self-balancing binary search tree (Red-Black Tree, AVL Tree) to skip list. In this book we will implement memtable using skip list. It seems that skip list is preferred in modern databases implementation (RocksDB, etc.) thanks to its simplicity and [concurrent access support](https://15721.courses.cs.cmu.edu/spring2016/papers/pugh-skiplists1990.pdf). Using tree or skip list limits the time complexity to O(log n) for insertion, search, deletion, etc.

- HBase implements memtable as skip list.
- CockroachDB uses the Pebble storage engine which is based on RocksDB and implements its memtable as a skip list.
- Apache Cassandra implements skip lists and a Trie memtable.
- RocksDB by default uses a skip list but also implements a hash link list, hash skip list, and a vector.

### Skip-list for search

### Read and write

Multiple memtables can be maintained in memory for concurrent reads. Only one memtable is available for write. When it is full and should be flushed, a new memtable takes the place. The full one is marked as immutable for read only and ready to be flushed as SSTable to durable storage. Each write is not flushed immediately. This is the delay write technique that is common in databases to reduce IO to durable storage which is object storage in our case (or disk on other database systems). In later chapters, we will work on the storage mechanism to persist the memtables.

## References
- https://ayende.com/blog/161793/ravens-storage-memtables-are-tough
- https://www.darchuletajr.com/blog/lsm-trees-memtables-sorted-string-tables-introduction
- https://www.mydistributed.systems/2021/03/skip-list-data-structure.html
- [Arxiv Paper. The Skiplist-based LSM Tree](https://arxiv.org/pdf/1809.03261)

