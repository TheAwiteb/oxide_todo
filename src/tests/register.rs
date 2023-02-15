use actix_web::{web, App};

use super::init_test_pool;
use crate::{auth, schemas::auth::RegisterSchema};

async fn register_request(username: String, password: String) -> super::TestResponseType {
    let user = RegisterSchema { username, password };
    let pool = init_test_pool().await;
    let srv = actix_test::start(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(auth::register::register)
    });
    srv.post("/register").send_json(&user).await.unwrap()
}

#[rstest::rstest]
#[case::register_with_correct_credentials("testusername1", "testpassword", 201)]
#[case::register_with_already_existing_username("testusername1", "testpassword", 400)]
#[case::empty_username("", "testpassword", 400)]
#[case::empty_password("testusername1", "", 400)]
#[case::empty_username_and_password("", "", 400)]
#[actix_web::test]
async fn register_endpoint(
    #[case] username: String,
    #[case] password: String,
    #[case] status_code: u16,
) {
    let res = register_request(username, password).await;
    super::check_content_type(&res);
    super::check_content_length(&res);
    assert_eq!(res.status(), status_code);
}
