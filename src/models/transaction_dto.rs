use lazy_static::lazy_static;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref RE_TWO_CHARS: Regex = Regex::new(r"c|d").unwrap();
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate, PartialEq)]
pub struct TransactionDTO {
    pub valor: i64,
    #[validate(regex = "RE_TWO_CHARS")]
    pub tipo: String,
    #[validate(length(min = 1, max = 10))]
    pub descricao: String,
}
