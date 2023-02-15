use crate::schemas::todo::TodoScheam;
use crate::schemas::user::UserSchema;
use crate::tests::login::login_req;
use crate::tests::{check_content_length, check_content_type, init_test_pool, TestResponseType};
use actix_web::{web, App};
use entity::todo::Status as TodoStatus;
use serde_json::json;
use uuid::Uuid;

use super::create_todo::create_todo_req;

pub async fn update_todo_req(uuid: Uuid, title: &str, status: &str) -> TestResponseType {
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
            .service(web::scope("/todo").service(crate::api::todo::update::update_todo))
    });
    srv.put(format!("todo/{uuid}"))
        .insert_header(("Authorization", format!("Bearer {}", user.token)))
        .send_json(&json! {{
            "title": title,
            "status": status
        }})
        .await
        .unwrap()
}

#[actix_web::test]
#[serial_test::serial]
async fn update_todo() {
    let mut todo = create_todo_req("new_todo_title".to_owned(), "completed".to_owned()).await;
    let todo: TodoScheam =
        serde_json::from_slice(todo.body().await.unwrap().to_vec().as_slice()).unwrap();
    let mut response = update_todo_req(todo.uuid, "some_new_todo_title", "pending").await;
    check_content_type(&response);
    check_content_length(&response);
    assert_eq!(response.status().as_u16(), 200);
    let new_todo: TodoScheam =
        serde_json::from_slice(response.body().await.unwrap().to_vec().as_slice()).unwrap();
    assert_eq!(new_todo.title, "some_new_todo_title");
    assert_eq!(new_todo.status, TodoStatus::Pending);
}

#[actix_web::test]
#[serial_test::serial]
async fn update_invalid_todo() {
    let response = update_todo_req(Uuid::new_v4(), "SomeT", "completed").await;
    check_content_type(&response);
    check_content_length(&response);
    assert_eq!(response.status().as_u16(), 404);
}
