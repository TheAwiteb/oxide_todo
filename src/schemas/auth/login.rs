use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::auth::utils as auth_utils;
use crate::errors::Result as TodoResult;
use crate::schemas::user::UserSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSchema {
    pub username: String,
    pub password: String,
}

impl LoginSchema {
    /// Login a user, return a token
    /// If the user does not exist, return an error
    pub async fn login(&self, db: &DatabaseConnection) -> TodoResult<UserSchema> {
        auth_utils::get_user_by_username_and_password(db, &self.username, &self.password)
            .await
            .map(UserSchema::try_from_model)?
    }
}
