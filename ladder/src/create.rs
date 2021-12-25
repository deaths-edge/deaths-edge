use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::account;

#[derive(Serialize, Deserialize)]
pub struct CreatePayload {
    name: String,
    username: String,
    password: String,
}

impl IntoResponse for CreateError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self.0 {
            DbErr::Conn(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            DbErr::Exec(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            DbErr::Query(err) => (StatusCode::BAD_REQUEST, err),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, String::new()),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub struct CreateError(DbErr);

pub async fn create(
    Json(payload): Json<CreatePayload>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<(), CreateError> {
    let account = account::ActiveModel {
        name: Set(payload.name),
        username: Set(payload.username),
        password: Set(payload.password),
        ..Default::default()
    };

    account.insert(conn).await.map_err(CreateError)?;

    Ok(())
}
