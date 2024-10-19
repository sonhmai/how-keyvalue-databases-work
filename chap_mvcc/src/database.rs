use mvcc::{IsolationLevel, Value};
use std::collections::HashMap;

pub struct Database {
    default_isolation: IsolationLevel,
    /// mapping of key to list of values, later value means newer version.
    store: HashMap<String, Vec<Value>>,
    next_transaction_id: usize,
}

impl Database {
    pub fn new() -> Self {
        Self {
            default_isolation: IsolationLevel::ReadCommitted,
            store: HashMap::new(),
            next_transaction_id: 1,
        }
    }
}