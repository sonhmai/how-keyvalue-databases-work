/// A value in the database
pub struct Value {
    tx_start_id: usize,
    tx_end_id: usize,
    value: String,
}

pub enum TransactionState {
    InProgress,
    Aborted,
    Committed,
}

#[derive(Debug, Clone, Copy)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Snapshot,
    Serializable,
}

pub struct Transaction {
    isolation_level: IsolationLevel,
    pub id: usize,
    state: TransactionState,
}

impl Transaction {
    pub fn new(
        isolation_level: IsolationLevel,
        id: usize,
        state: TransactionState,
    ) -> Transaction {
        Transaction { isolation_level, id, state }
    }

    pub fn get(&self, key: &str) -> String {
        "".to_string()
    }

    pub fn set(&self, key: &str, value: &str) -> () {

    }
}