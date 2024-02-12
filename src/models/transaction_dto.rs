use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionDTO {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
}
