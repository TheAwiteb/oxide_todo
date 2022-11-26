use std::collections::BTreeMap;

use actix_web::HttpRequest;
use entity::user::{Column as UserColumn, Entity as User, Model as UserModel};
use hmac::{Hmac, Mac};
use jwt::{header::HeaderType, Header, SignWithKey, Token};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sha2::{Digest, Sha256};

use crate::auth::traits::ClaimsHelper;
use crate::errors::TodoError as TodoErrorTrait;
use crate::errors::{Error as TodoError, Result as TodoResult};

/// Hash given data by sha256 algorithm.
pub fn hash_function(data: &str) -> String {
    hex::encode(Sha256::digest(data))
}

/// Generate a JWT token for a user
/// ### Arguments
/// * `user_id` - The id of the user
/// * `created_date` - The timestamp of the token created, to add it to JWT payload
pub fn generate_token(user_id: u32, created_date: i64) -> TodoResult<String> {
    let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let str_id = user_id.to_string();
    let str_timestamp = created_date.to_string();
    let payload: BTreeMap<&str, &str> =
        [("id", str_id.as_str()), ("created_at", &str_timestamp)].into();
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).key_creation_err()?;
    let header = jwt::Header {
        type_: Some(HeaderType::JsonWebToken),
        ..Default::default()
    };
    jwt::Token::new(header, payload)
        .sign_with_key(&key)
        .map(Into::into)
        .key_creation_err()
}

/// Return the user by given token
/// ### Errors
/// - User not found
/// - Token is invalid
/// - Token is was revoked
pub async fn get_user_by_token(db: &DatabaseConnection, token: &str) -> TodoResult<UserModel> {
    let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).key_creation_err()?;
    let claims = jwt::VerifyWithKey::<Token<Header, BTreeMap<String, String>, _>>::verify_with_key(
        token, &key,
    )
    .invalid_token_err()?
    .claims()
    .clone();
    let user = User::find_by_id(claims.get_user_id())
        .one(db)
        .await
        .database_err()?
        .incorrect_user_err()?;

    if claims.get_created_at() != user.token_created_at {
        Err(TodoError::Forbidden("Token has been revoked".to_owned()))
    } else {
        Ok(user)
    }
}

/// Return the user by given username and password, or return an error if the user does not exist.
pub async fn get_user_by_username_and_password(
    db: &DatabaseConnection,
    username: &str,
    password: &str,
) -> TodoResult<UserModel> {
    let hashed_password = hash_function(password);

    User::find()
        .filter(
            UserColumn::Name
                .eq(username)
                .and(UserColumn::HashedPassword.eq(hashed_password)),
        )
        .one(db)
        .await
        .database_err()?
        .incorrect_user_err()
}

/// Extract the token from the request header.
pub fn extract_token(req: &HttpRequest) -> TodoResult<String> {
    req.headers()
        .get("Authorization")
        .map(|token| token.to_str().map(|token| token.strip_prefix("Bearer ")))
        .bad_request_err("`Authorization` header is missing")?
        .bad_request_err("The token is invalid, cannot convert it to string")?
        .bad_request_err("Token should start with `Bearer` prefix")
        .map(|token| token.to_owned())
}

/// Return user model by given request
pub async fn req_auth(req: HttpRequest, db: &DatabaseConnection) -> TodoResult<UserModel> {
    let token = extract_token(&req)?;

    get_user_by_token(db, &token).await
}
