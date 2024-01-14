use std::collections::HashMap;
use warp::Rejection;
use warp::Reply;

use crate::profanity::check_profanity;
use crate::store::Store;
use crate::types::pagination::Pagination;
use crate::types::question::{NewQuestion, Question};
use tracing::{event, instrument, Level};
use warp::http::StatusCode;
//Instrument macro (https://tracing.rs/tracing/attr.instrument.html)
//to auto- matically open and close a span when the function is called

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "rust_faq_webapp", Level::INFO, "querying questions");
    let pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        // let pagination = extract_pagination(params)?;
    }
    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = tokio::spawn(check_profanity(question.title));
    let content = tokio::spawn(check_profanity(question.content));

    let (title, content) = (title.await.unwrap(), content.await.unwrap());

    if title.is_err() {
        return  Err(warp::reject::custom(title.unwrap_err()));
    }
    if content.is_err() {
        return  Err(warp::reject::custom(title.unwrap_err()));
    }

    let question = Question {
        id: question.id,
        title: title.unwrap(),
        content: content.unwrap(),
        tags: question.tags,
    };

    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl Reply, Rejection> {
    let title = match check_profanity(new_question.title).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let content = match check_profanity(new_question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    match store.add_question(question).await {
        Ok(_) => Ok(warp::reply::with_status("Question Added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
