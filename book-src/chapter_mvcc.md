# Chapter MVCC

A transaction has
1. an isolation level
2. a monotonic increasing int
3. current state

What
- non-locking concurrency control mechanism

How
- Multi versions of a value is stored with timestamp or transaction id.
- When value is modified, not changing it in place but `create a newer version` of it.
- Concurrent reads access older value.
- Writes create newer version of a value.
- most commonly isolation level: `snapshot isolation`

Pros
1. No locking needed.

Challenges
1. more storage due to storing more data.
2. how to remove obsolete older versions (never read again).

References
- https://notes.eatonphil.com/2024-05-16-mvcc.html
- https://skyzh.github.io/mini-lsm/week3-overview.html
- https://tikv.org/deep-dive/key-value-engine/rocksdb/
- https://en.wikipedia.org/wiki/Multiversion_concurrency_control