use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::{account, team2, Team2};

#[derive(Serialize, Deserialize)]
pub struct CreatePayload {
    name: String,
    member_1: String,
    member_2: String,
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

pub async fn create_team(
    Json(payload): Json<CreatePayload>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<(), CreateError> {
    let member_1 = Team2::find()
        .filter(account::Column::Username.contains(&payload.member_1))
        .one(conn)
        .await
        .map_err(CreateError)?
        .expect("TODO")
        .id;
    let member_2 = Team2::find()
        .filter(account::Column::Username.contains(&payload.member_1))
        .one(conn)
        .await
        .map_err(CreateError)?
        .expect("TODO")
        .id;
    let account = team2::ActiveModel {
        name: Set(payload.name),
        member_1: Set(member_1),
        member_2: Set(member_2),
        ..Default::default()
    };

    account.insert(conn).await.map_err(CreateError)?;

    Ok(())
}
