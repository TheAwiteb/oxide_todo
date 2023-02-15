use actix_web::{body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder};
use entity::user::{ActiveModel, Model as UserModel};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::auth::utils as auth_utils;
use crate::errors::Result as ApiResult;

/// The schema of the user response
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UserSchema {
    /// The name of the user
    #[serde(rename = "username")]
    #[schema(example = "Awiteb")]
    pub name: String,
    /// The token of the user
    #[schema(
        example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdXN0IjoidGVzdCJ9.rCOJrqm0MQvxTcNyyXJdqI7smV7VNvAJZ02LslGkL78"
    )]
    pub token: String,
    #[serde(skip)]
    #[schema(hidden)]
    code: StatusCode,
}

impl UserSchema {
    pub fn new(name: String, token: String) -> Self {
        Self {
            name,
            token,
            code: StatusCode::OK,
        }
    }

    #[allow(dead_code)]
    pub async fn try_into_model(self, db: &DatabaseConnection) -> ApiResult<UserModel> {
        auth_utils::get_user_by_token(db, &self.token).await
    }

    /// Create a user schema from a user active model, will generate a token
    pub fn try_from_active_model(user: ActiveModel) -> ApiResult<Self> {
        // Here `unwrap` means extrct the value from the `ActiveValue`
        // See https://docs.rs/sea-orm/0.10.4/sea_orm/entity/enum.ActiveValue.html#method.unwrap
        auth_utils::generate_token(user.id.unwrap(), user.token_created_at.unwrap())
            .map(|token| Self::new(user.name.unwrap(), token))
    }

    /// Create a user schema from a user model, will generate a token
    pub fn try_from_model(user: UserModel) -> ApiResult<Self> {
        auth_utils::generate_token(user.id, user.token_created_at)
            .map(|token| Self::new(user.name, token))
    }

    /// Set the status code of the response
    pub fn with_code(mut self, code: StatusCode) -> Self {
        self.code = code;
        self
    }
}

impl Responder for UserSchema {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(self.code).json(self)
    }
}
