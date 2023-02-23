use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
struct BookStoreError {
    status: u16,
    message: String,
}

pub enum BookStoreResponse<T: Serialize> {
    Ok((StatusCode, Json<T>)),
    Err((StatusCode, String)),
}

impl<T: Serialize> IntoResponse for BookStoreResponse<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            BookStoreResponse::Ok(v) => v.into_response(),
            BookStoreResponse::Err((status, err)) => {
                let error = BookStoreError {
                    status: StatusCode::as_u16(&status),
                    message: err,
                };
                (status, Json::from(error)).into_response()
            }
        }
    }
}
