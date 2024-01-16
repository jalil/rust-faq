use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: AccountId,
    //nbf, which means “not used before this timestamp.”
    pub nbf: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: Option<AccountId>,
    pub email: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone)]
pub struct AccountId(pub i32);
