use serde_derive::{Deserialize, Serialize};

use crate::repository::transaction::Transaction;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Extract {
    pub saldo: Saldo,
    pub ultimas_transacoes: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Saldo {
    pub total: i64,
    pub data_extrato: String,
    pub limite: i64,
}
