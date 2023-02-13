use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::auth::utils as auth_utils;
use crate::errors::Result as ApiResult;
use crate::schemas::user::UserSchema;

/// The schema for login request
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct LoginSchema {
    /// The username of the user
    #[schema(example = "Awiteb")]
    pub username: String,
    /// The password of the user
    #[schema(example = "123456")]
    pub password: String,
}

impl LoginSchema {
    /// Login a user, return a token
    /// If the user does not exist, return an error
    pub async fn login(&self, db: &DatabaseConnection) -> ApiResult<UserSchema> {
        auth_utils::get_user_by_username_and_password(db, &self.username, &self.password)
            .await
            .map(UserSchema::try_from_model)?
    }
}
