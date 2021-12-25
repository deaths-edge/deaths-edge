use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::KEYS;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
}

#[derive(Debug)]
pub struct InvalidToken;

impl IntoResponse for InvalidToken {
    fn into_response(self) -> Response {
        let (status, error_message) = (StatusCode::BAD_REQUEST, "Invalid token");
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = InvalidToken;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, InvalidToken> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| InvalidToken)?;

        Ok(token_data.claims)
    }
}
