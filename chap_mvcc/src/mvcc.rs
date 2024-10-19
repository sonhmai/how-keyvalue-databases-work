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

pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Snapshot,
    Serializable,
}

pub struct Transaction {
    isolation_level: IsolationLevel,
    id: usize,
    state: TransactionState,
}