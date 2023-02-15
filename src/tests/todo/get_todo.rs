use crate::schemas::todo::{TodoListSchema, TodoScheam};
use crate::schemas::user::UserSchema;
use crate::tests::login::login_req;
use crate::tests::todo::list_todo::list_todo_req;
use crate::tests::{check_content_length, check_content_type, init_test_pool, TestResponseType};
use actix_web::{web, App};
use uuid::Uuid;

pub async fn get_todo_req(uuid: Uuid) -> TestResponseType {
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
            .service(web::scope("/todo").service(crate::todo::get_todo::get_todo))
    });
    srv.get(format!("todo/{uuid}"))
        .insert_header(("Authorization", format!("Bearer {}", user.token)))
        .send()
        .await
        .unwrap()
}

#[actix_web::test]
#[serial_test::serial]
async fn get_valid_todo() {
    let todos: TodoListSchema = serde_json::from_slice(
        list_todo_req("")
            .await
            .body()
            .await
            .unwrap()
            .to_vec()
            .as_slice(),
    )
    .unwrap();
    for todo in todos.data {
        let mut response = get_todo_req(todo.uuid).await;
        check_content_type(&response);
        check_content_length(&response);
        assert_eq!(response.status().as_u16(), 200);

        let todo_from_res: TodoScheam =
            serde_json::from_slice(response.body().await.unwrap().to_vec().as_slice()).unwrap();
        assert_eq!(todo_from_res.uuid, todo.uuid);
        assert_eq!(todo_from_res.title, todo.title);
        assert_eq!(todo_from_res.status, todo.status);
        assert_eq!(todo_from_res.created_at, todo.created_at);
        assert_eq!(todo_from_res.updated_at, todo.updated_at);
    }
}

#[actix_web::test]
#[serial_test::serial]
async fn get_invalid_todo() {
    let response = get_todo_req(Uuid::new_v4()).await;
    check_content_type(&response);
    check_content_length(&response);
    assert_eq!(response.status().as_u16(), 404);
}
