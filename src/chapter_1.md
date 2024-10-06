# Chapter 1 Overview

Three main components of a key-value databases are
1. memtable
2. sorted-string table
3. WAL (write-ahead log) file

`memtable` is an in-memory data structure that serves reads and writes. New writes go to the wal for persistence. The WAL is regularly sends to object storage for durability.

