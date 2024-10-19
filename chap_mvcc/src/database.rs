use mvcc::{IsolationLevel, Transaction, TransactionState, Value};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Database {
    pub default_isolation: IsolationLevel,
    /// mapping of key to list of values, later value means newer version.
    store: HashMap<String, Vec<Value>>,
    transactions: HashMap<usize, Rc<Transaction>>,
    next_transaction_id: usize,
}

impl Database {
    pub fn new() -> Self {
        Self {
            default_isolation: IsolationLevel::ReadCommitted,
            store: HashMap::new(),
            transactions: HashMap::new(),
            next_transaction_id: 1,
        }
    }

    pub fn new_transaction(&mut self) -> Rc<Transaction> {
        let tid = self.next_transaction_id;;
        let t = Transaction::new(
            self.default_isolation,
            tid,
            TransactionState::InProgress,
        );
        self.next_transaction_id += 1;
        let rc = Rc::new(t);
        self.transactions.insert(tid, rc.clone());

        rc.clone()
    }
}

#[cfg(test)]
mod tests {
    use database::Database;
    use mvcc::IsolationLevel;

    #[test]
    fn read_uncommitted() {
        let mut db = Database::new();
        db.default_isolation = IsolationLevel::ReadUncommitted;

        let t1 = db.new_transaction();
        let t2 = db.new_transaction();

        t1.set("x", "hey");

        // update seen by client 1
        let val1 = t1.get("x");
        assert_eq!(val1, "hey");

        // since read uncommitted, other clients see this uncommitted values.
        // This is mostly unwanted in production scenario.
        let val2 = t2.get("x");
        assert_eq!(val2, "hey");
    }
}