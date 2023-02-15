use crate::schemas::todo::TodoListSchema;
use crate::schemas::user::UserSchema;
use crate::tests::login::login_req;
use crate::tests::todo::list_todo::list_todo_req;
use crate::tests::{check_content_length, check_content_type, init_test_pool, TestResponseType};
use actix_web::{web, App};
use uuid::Uuid;

use super::create_todo::create_todo_req;

pub async fn delete_todo_req(uuid: Uuid) -> TestResponseType {
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
            .service(web::scope("/todo").service(crate::api::todo::delete_todo::delete_todo))
    });
    srv.delete(format!("todo/{uuid}"))
        .insert_header(("Authorization", format!("Bearer {}", user.token)))
        .send()
        .await
        .unwrap()
}

#[actix_web::test]
#[serial_test::serial]
async fn delete_todo() {
    let get_list = || async {
        serde_json::from_slice::<TodoListSchema>(
            list_todo_req("")
                .await
                .body()
                .await
                .unwrap()
                .to_vec()
                .as_slice(),
        )
        .unwrap()
    };

    let new_todo_title = "NewTodoFromDelete";
    create_todo_req(new_todo_title.to_owned(), "completed".to_owned()).await;
    let todos = get_list().await;
    assert!(
        todos.data.iter().any(|t| t.title == new_todo_title),
        "The todo was not created"
    );
    let response = delete_todo_req(
        todos
            .data
            .iter()
            .find(|t| t.title == new_todo_title)
            .unwrap()
            .uuid,
    )
    .await;
    check_content_type(&response);
    check_content_length(&response);
    assert_eq!(response.status().as_u16(), 200);
    let todos = get_list().await;
    assert!(
        !todos.data.iter().any(|t| t.title == new_todo_title),
        "The todo was not deleted"
    );
}

#[actix_web::test]
#[serial_test::serial]
async fn delete_invalid_todo() {
    let response = delete_todo_req(Uuid::new_v4()).await;
    check_content_type(&response);
    check_content_length(&response);
    assert_eq!(response.status().as_u16(), 404);
}
