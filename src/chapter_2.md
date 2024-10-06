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