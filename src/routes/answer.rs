use crate::store::Store;
use crate::types::answer::NewAnswer;
use warp::http::StatusCode;
//find a way to generate unique IDs when creating new answers.
//use of unwrap here, which is not production-ready code.
//If we can’t find a parameter, the Rust application will panic and crash.
//think about using match here instead and returning each error case for a missing parameter separately
//
pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_answer(new_answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
