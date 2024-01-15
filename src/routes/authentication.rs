use warp::http::StatusCode;

use crate::store::Store;
use crate::types::account::Account;
use warp::Rejection;
use warp::Reply;

use argon2::{self, Config};
use rand::Rng;
pub async fn register(store: Store, account: Account) -> Result<impl Reply, Rejection> {
    let hashed_password = hashed_password(account.password.as_bytes());
    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };
    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub fn hashed_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}
