# Chapter 3 Storage Engine

Contents
- How is storage engine implemented with SSTable and LSM tree

> a nice overview picture taken from https://kflu.github.io/2018/02/09/2018-02-09-lsm-tree/

![img.png](img.png)

Quoting this good piece from [David Archuleta Jr.'s Blog - LSM Trees, Memtables & Sorted String Tables: An Introduction:](https://www.darchuletajr.com/blog/lsm-trees-memtables-sorted-string-tables-introduction)

> The on-disk component of the LSM Tree is the Sorted String Table. In order to write the contents of a sorted data structure, whether a tree, skip list, vector, or any other data structure, you just need to iterate in order from the lowest value to the highest. So it is a very simple and straightforward linear operation.

## Sorted String Table

The bad thing about just appending data to log file is that it is going to be slow as the database grows. Each time we want to look for a key, we need to traverse the entire data segment file to look for out key.

One novel idea to improve that is to `sort the data in each segment by key`. Actually this idea is not so novel as it is already widely known in Computer Science for a long time and is used in various data structure for example sorting and searching (binary search specifically). Sorting the keys can help us reduce the time complexity from O(n) to only O(log n). We call this new idea implementation **Sorted String Tables**.

Sorted Strings Table (SSTable) is a file format commonly used by NoSQL databases to store the data flushed by in-memory memtables to durable storage. It can be generally understood as a serialization format for the memtables. The memtables serialize the data when flushing and deserialize the binary back to memory structures.

### Log-Structure Merge Tree

An SSTable uses a Log-Structured Merge (LSM) tree data structure format. This format is more efficient for write-heavy, extremely large data sets than a traditional B-tree (pronounced “Bee tree”) format which commonly used in relational databases.

### Reading from SSTable and bloom filter

bloom filter having its own module now

## References
- https://www.scylladb.com/glossary/sstable/
- https://jyotinder.substack.com/p/building-a-write-optimized-database-part-3
- https://kflu.github.io/2018/02/09/2018-02-09-lsm-tree/
- 
