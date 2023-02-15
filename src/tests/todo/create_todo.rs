use crate::errors::Error as ApiError;
use actix_web::web::JsonConfig;
use actix_web::{web, App};
use serde_json::json;

use crate::schemas::user::UserSchema;
use crate::tests::login::login_req;
use crate::tests::{check_content_length, check_content_type, init_test_pool, TestResponseType};

pub async fn create_todo_req(title: String, status: String) -> TestResponseType {
    let pool = init_test_pool().await;
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
    let srv = actix_test::start(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(JsonConfig::default().error_handler(|err, _| ApiError::from(err).into()))
            .service(web::scope("/todo").service(crate::api::todo::create::create))
    });
    srv.post("/todo")
        .insert_header(("Authorization", format!("Bearer {}", user.token)))
        .send_json(&json!({"title": title, "status": status}))
        .await
        .unwrap()
}

#[rstest::rstest]
#[case::create_todo_with_correct_credentials("testtitle1", "completed", 200)]
#[case::create_todo_with_correct_credentials("testtitle2", "pending", 200)]
#[case::create_todo_with_correct_credentials("testtitle3", "progress", 200)]
#[case::create_todo_with_correct_credentials("testtitle4", "cancelled", 200)]
#[case::create_todo_with_existing_title("testtitle1", "completed", 400)]
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
    let create_todo_res = create_todo_req(title, status).await;
    check_content_type(&create_todo_res);
    check_content_length(&create_todo_res);
    assert_eq!(create_todo_res.status(), status_code);
    // if the status code is 200, wait for 1 second to make a time lag to be tested in list todo endpoint
    if status_code == 200 {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
