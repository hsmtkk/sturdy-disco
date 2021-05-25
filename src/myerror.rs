use thiserror::Error;
use actix_web::ResponseError;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

impl ResponseError for MyError {}
