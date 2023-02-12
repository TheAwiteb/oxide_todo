use actix_web::{web, App};
use serde_json::json;

use crate::schemas::user::UserSchema;
use crate::tests::login::login_req;
use crate::tests::{check_content_length, check_content_type, init_test_pool, TestResponseType};
use crate::todo;

pub async fn create_todo_req(title: String, status: String, token: String) -> TestResponseType {
    let pool = init_test_pool().await;
    let srv = actix_test::start(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(todo::create::create)
    });
    srv.post("/")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .send_json(&json!({"title": title, "status": status}))
        .await
        .unwrap()
}

#[rstest::rstest]
#[case::create_todo_with_correct_credentials("testtitle1", "completed", 200)]
#[case::create_todo_with_empty_title("", "pending", 400)]
#[case::create_todo_with_empty_status("testtitle2", "", 400)]
#[case::create_todo_with_empty_title_and_status("", "", 400)]
#[case::create_todo_with_invalid_status("testtitle3", "teststatus", 400)]
#[actix_web::test]
#[serial_test::serial]
async fn create_todo_endpoint(
    #[case] title: String,
    #[case] status: String,
    #[case] status_code: u16,
) {
    let user: UserSchema = serde_json::from_slice(
        login_req("testusername1".to_owned(), "testpassword".to_owned())
            .await
            .body()
            .await
            .unwrap()
            .to_vec()
            .as_slice(),
    )
    .unwrap();

    let create_todo_res = create_todo_req(title, status, user.token).await;
    assert_eq!(create_todo_res.status(), status_code);
    check_content_type(&create_todo_res);
    check_content_length(&create_todo_res);
}
