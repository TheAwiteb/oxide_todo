use crate::schemas::todo::TodoListSchema;
use crate::schemas::user::UserSchema;
use crate::tests::login::login_req;
use crate::tests::todo::list_todo::list_todo_req;
use crate::tests::{check_content_length, check_content_type, init_test_pool, TestResponseType};
use actix_web::{web, App};

pub async fn delete_todos_req() -> TestResponseType {
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
            .service(web::scope("/todo").service(crate::api::todo::delete_todos::delete_todos))
    });
    srv.delete("todo")
        .insert_header(("Authorization", format!("Bearer {}", user.token)))
        .send()
        .await
        .unwrap()
}

#[actix_web::test]
#[serial_test::serial]
async fn delete_todos() {
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

    let todos = get_list().await;
    assert!(!todos.data.is_empty(), "There are no todos to delete");
    let response = delete_todos_req().await;
    check_content_type(&response);
    check_content_length(&response);
    assert_eq!(response.status(), 200);
    let todos = get_list().await;
    assert!(todos.data.is_empty(), "Todos were not deleted");
}
