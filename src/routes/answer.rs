use crate::store::Store;
use crate::types::{
    answer::{Answer, AnswerId},
    question::QuestionId,
};
use std::collections::HashMap;
use warp::http::StatusCode;
//find a way to generate unique IDs when creating new answers.
//use of unwrap here, which is not production-ready code.
//If we can’t find a parameter, the Rust application will panic and crash.
//think about using match here instead and returning each error case for a missing parameter separately
//
pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("QuestionId").unwrap().to_string()),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer Added", StatusCode::OK))
}
