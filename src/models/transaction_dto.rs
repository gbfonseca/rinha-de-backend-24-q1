use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionDTO {
    pub valor: i64,
    pub tipo: String,
    pub descricao: String,
}
