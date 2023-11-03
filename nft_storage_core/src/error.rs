use nft_storage_api::{
    apis::nft_storage_api::{
        CheckError, DeleteError, DidGetError, ListError, StatusError, StoreError,
        UcanTokenPostError, UploadError, UserDidPostError,
    },
    models::{
        DidNotFoundErrorResponse, ErrorResponse, ForbiddenErrorResponse, UnauthorizedErrorResponse,
    },
};
use thiserror::Error;

use nft_storage_api::apis::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Bad request: {0}")]
    BadRequest(ErrorResponse),

    #[error("Unauthorized: {0}")]
    Unauthorized(UnauthorizedErrorResponse),

    #[error("Forbidden: {0}")]
    Forbidden(ForbiddenErrorResponse),

    #[error("Not found: {0}")]
    NotFound(ErrorResponse),

    #[error("Server error: {0}")]
    DidNotFound(DidNotFoundErrorResponse),

    #[error("Server error: {0}")]
    ServerError(ErrorResponse),

    #[error("Client error: {0}")]
    ClientError(String),

    #[error("Unknown value: {0}")]
    UnknownValue(serde_json::Value),

    #[error("API key missing")]
    ApiKeyMissing,
}

impl From<Error<CheckError>> for CoreError {
    fn from(error: Error<CheckError>) -> Self {
        match error {
            Error::Reqwest(e) => CoreError::ClientError(e.to_string()),
            Error::Serde(e) => CoreError::ClientError(e.to_string()),
            Error::Io(e) => CoreError::ClientError(e.to_string()),
            Error::ResponseError(content) => match &content.entity.unwrap() {
                CheckError::Status404(resp) => CoreError::NotFound(resp.clone()),
                CheckError::Status500(resp) => CoreError::ServerError(resp.clone()),
                CheckError::UnknownValue(val) => CoreError::UnknownValue(val.clone()),
            },
        }
    }
}

impl From<Error<DeleteError>> for CoreError {
    fn from(error: Error<DeleteError>) -> Self {
        match error {
            Error::Reqwest(e) => CoreError::ClientError(e.to_string()),
            Error::Serde(e) => CoreError::ClientError(e.to_string()),
            Error::Io(e) => CoreError::ClientError(e.to_string()),
            Error::ResponseError(content) => match content.entity {
                Some(DeleteError::Status401(resp)) => CoreError::Unauthorized(resp),
                Some(DeleteError::Status403(resp)) => CoreError::Forbidden(resp),
                Some(DeleteError::Status500(resp)) => CoreError::ServerError(resp),
                Some(DeleteError::UnknownValue(val)) => CoreError::UnknownValue(val),
                None => CoreError::UnknownValue(serde_json::Value::String(
                    "Unexpected error".to_string(),
                )),
            },
        }
    }
}

impl From<Error<DidGetError>> for CoreError {
    fn from(error: Error<DidGetError>) -> Self {
        match error {
            Error::Reqwest(e) => CoreError::ClientError(e.to_string()),
            Error::Serde(e) => CoreError::ClientError(e.to_string()),
            Error::Io(e) => CoreError::ClientError(e.to_string()),
            Error::ResponseError(content) => match content.entity {
                Some(DidGetError::UnknownValue(val)) => CoreError::UnknownValue(val),
                None => CoreError::UnknownValue(serde_json::Value::String(
                    "Unexpected error".to_string(),
                )),
            },
        }
    }
}

impl From<Error<ListError>> for CoreError {
    fn from(error: Error<ListError>) -> Self {
        match error {
            Error::Reqwest(e) => CoreError::ClientError(e.to_string()),
            Error::Serde(e) => CoreError::ClientError(e.to_string()),
            Error::Io(e) => CoreError::ClientError(e.to_string()),
            Error::ResponseError(content) => match content.entity {
                Some(ListError::Status401(resp)) => CoreError::Unauthorized(resp),
                Some(ListError::Status403(resp)) => CoreError::Forbidden(resp),
                Some(ListError::Status500(resp)) => CoreError::ServerError(resp),
                Some(ListError::UnknownValue(val)) => CoreError::UnknownValue(val),
                None => CoreError::UnknownValue(serde_json::Value::String(
                    "Unexpected error".to_string(),
                )),
            },
        }
    }
}

impl From<Error<StatusError>> for CoreError {
    fn from(error: Error<StatusError>) -> Self {
        match error {
            Error::Reqwest(e) => CoreError::ClientError(e.to_string()),
            Error::Serde(e) => CoreError::ClientError(e.to_string()),
            Error::Io(e) => CoreError::ClientError(e.to_string()),
            Error::ResponseError(content) => match content.entity {
                Some(StatusError::Status401(resp)) => CoreError::Unauthorized(resp),
                Some(StatusError::Status403(resp)) => CoreError::Forbidden(resp),
                Some(StatusError::Status500(resp)) => CoreError::ServerError(resp),
                Some(StatusError::UnknownValue(val)) => CoreError::UnknownValue(val),
                None => CoreError::UnknownValue(serde_json::Value::String(
                    "Unexpected error".to_string(),
                )),
            },
        }
    }
}

impl From<Error<StoreError>> for CoreError {
    fn from(error: Error<StoreError>) -> Self {
        match error {
            Error::Reqwest(e) => CoreError::ClientError(e.to_string()),
            Error::Serde(e) => CoreError::ClientError(e.to_string()),
            Error::Io(e) => CoreError::ClientError(e.to_string()),
            Error::ResponseError(content) => match content.entity {
                Some(StoreError::Status400(resp)) => CoreError::BadRequest(resp),
                Some(StoreError::Status401(resp)) => CoreError::Unauthorized(resp),
                Some(StoreError::Status403(resp)) => CoreError::Forbidden(resp),
                Some(StoreError::Status500(resp)) => CoreError::ServerError(resp),
                Some(StoreError::UnknownValue(val)) => CoreError::UnknownValue(val),
                None => CoreError::UnknownValue(serde_json::Value::String(
                    "Unexpected error".to_string(),
                )),
            },
        }
    }
}

impl From<Error<UcanTokenPostError>> for CoreError {
    fn from(error: Error<UcanTokenPostError>) -> Self {
        match error {
            Error::Reqwest(e) => CoreError::ClientError(e.to_string()),
            Error::Serde(e) => CoreError::ClientError(e.to_string()),
            Error::Io(e) => CoreError::ClientError(e.to_string()),
            Error::ResponseError(content) => match &content.entity {
                Some(UcanTokenPostError::Status400(resp)) => CoreError::DidNotFound(resp.clone()),
                Some(UcanTokenPostError::Status500(resp)) => CoreError::ServerError(resp.clone()),
                Some(UcanTokenPostError::UnknownValue(val)) => CoreError::UnknownValue(val.clone()),
                None => CoreError::UnknownValue(serde_json::Value::String(
                    "Unexpected error".to_string(),
                )),
            },
        }
    }
}

impl From<Error<UploadError>> for CoreError {
    fn from(error: Error<UploadError>) -> Self {
        match error {
            Error::Reqwest(e) => CoreError::ClientError(e.to_string()),
            Error::Serde(e) => CoreError::ClientError(e.to_string()),
            Error::Io(e) => CoreError::ClientError(e.to_string()),
            Error::ResponseError(content) => match content.entity {
                Some(UploadError::Status400(resp)) => CoreError::BadRequest(resp),
                Some(UploadError::Status401(resp)) => CoreError::Unauthorized(resp),
                Some(UploadError::Status403(resp)) => CoreError::Forbidden(resp),
                Some(UploadError::Status500(resp)) => CoreError::ServerError(resp),
                Some(UploadError::UnknownValue(val)) => CoreError::UnknownValue(val),
                None => CoreError::UnknownValue(serde_json::Value::String(
                    "Unexpected error".to_string(),
                )),
            },
        }
    }
}

impl From<Error<UserDidPostError>> for CoreError {
    fn from(error: Error<UserDidPostError>) -> Self {
        match error {
            Error::Reqwest(e) => CoreError::ClientError(e.to_string()),
            Error::Serde(e) => CoreError::ClientError(e.to_string()),
            Error::Io(e) => CoreError::ClientError(e.to_string()),
            Error::ResponseError(content) => match content.entity {
                Some(UserDidPostError::Status400(resp)) => CoreError::BadRequest(resp),
                Some(UserDidPostError::Status500(resp)) => CoreError::ServerError(resp),
                Some(UserDidPostError::UnknownValue(val)) => CoreError::UnknownValue(val),
                None => CoreError::UnknownValue(serde_json::Value::String(
                    "Unexpected error".to_string(),
                )),
            },
        }
    }
}
