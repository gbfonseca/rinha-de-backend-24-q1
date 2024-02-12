use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResult {
    pub limite: i32,
    pub saldo: i32,
}
