use std::borrow::Cow;

use chrono::Utc;
use entity::user::ActiveModel as NewUser;
use sea_orm::error::DbErr;
use sea_orm::{ActiveModelTrait, DatabaseConnection, RuntimeErr, Set};
use serde::{Deserialize, Serialize};
use sqlx_core::error::Error as SqlxError;
use utoipa::ToSchema;

use crate::api::auth::utils as auth_utils;
use crate::errors::{Error as ApiError, Result as ApiResult};
use crate::schemas::user::UserSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RegisterSchema {
    /// The name of the new user, should be unique
    #[schema(example = "Awiteb")]
    pub username: String,
    /// The password of the new user
    #[schema(example = "123456")]
    pub password: String,
}

impl RegisterSchema {
    pub async fn create(&self, db: &DatabaseConnection) -> ApiResult<UserSchema> {
        if self.username.is_empty() || self.password.is_empty() {
            return Err(ApiError::BAdRequest(
                "Invalid username or password, must be not empty".to_owned(),
            ));
        }

        let hashed_password = auth_utils::hash_function(&self.password);
        let current_time = Utc::now().naive_utc().timestamp();

        let user = NewUser {
            name: Set(self.username.clone()),
            hashed_password: Set(hashed_password),
            token_created_at: Set(current_time),
            created_at: Set(current_time),
            ..Default::default()
        }
        .save(db)
        .await
        .map_err(|db_err| {
            if let DbErr::Exec(RuntimeErr::SqlxError(SqlxError::Database(e))) = db_err {
                if e.code() == Some(Cow::Borrowed("2067")) {
                    return ApiError::BAdRequest(format!(
                        "Username {} already exists",
                        self.username
                    ));
                }
            }
            ApiError::InternalServer("Database error ):".to_owned())
        })?;

        UserSchema::try_from_active_model(user)
    }
}
