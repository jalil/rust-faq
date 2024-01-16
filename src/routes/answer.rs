use reqwest::StatusCode;
use crate::profanity::check_profanity;
use crate::store::Store;
use crate::types::account::Session;
use crate::types::answer::NewAnswer;
//find a way to generate unique IDs when creating new answers.
//use of unwrap here, which is not production-ready code.
//If we can’t find a parameter, the Rust application will panic and crash.
//think about using match here instead and returning each error case for a missing parameter separately
//
pub async fn add_answer(
    session: Session,
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    let content = match check_profanity(new_answer.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let answer = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };
    match store.add_answer(answer, account_id).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
