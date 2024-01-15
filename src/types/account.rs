use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: Option<AccountId>,
    pub email: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone)]
struct AccountId(pub i32);
