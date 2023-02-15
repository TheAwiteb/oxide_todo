use crate::errors::Error as ApiError;
use crate::tests::{check_content_length, check_content_type};
use crate::{
    schemas::{todo::TodoListSchema, user::UserSchema},
    tests::{init_test_pool, login::login_req, TestResponseType},
};
use actix_web::{
    web::{self, QueryConfig},
    App,
};
use entity::todo::Status;
use std::{cmp::Ordering, str::FromStr};

pub async fn list_todo_req(params: &str) -> TestResponseType {
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
            .app_data(QueryConfig::default().error_handler(|err, _| ApiError::from(err).into()))
            .service(web::scope("/todo").service(crate::api::todo::list::list))
    });
    srv.get(format!("/todo?{}", params))
        .insert_header(("Authorization", format!("Bearer {}", user.token)))
        .send()
        .await
        .unwrap()
}

#[rustfmt::skip]
#[rstest::rstest]
#[case::list_todo_with_correct_credentials("", 200, None, 4, 4, 0, 10)]
#[case::list_todo_with_correct_credentials("offset=1", 200, None, 4, 3, 1, 10)]
#[case::list_todo_with_correct_credentials("offset=2", 200, None, 4, 2, 2, 10)]
#[case::list_todo_with_correct_credentials("offset=3", 200, None, 4, 1, 3, 10)]
#[case::list_todo_with_correct_credentials("offset=4", 200, None, 4, 0, 4, 10)]
#[case::list_todo_with_correct_credentials("offset=5", 200, None, 4, 0, 5, 10)]
#[case::list_todo_with_correct_credentials("limit=1", 200, None, 4, 1, 0, 1)]
#[case::list_todo_with_correct_credentials("limit=2", 200, None, 4, 2, 0, 2)]
#[case::list_todo_with_correct_credentials("limit=3", 200, None, 4, 3, 0, 3)]
#[case::list_todo_with_correct_credentials("limit=4", 200, None, 4, 4, 0, 4)]
#[case::list_todo_with_correct_credentials("limit=5", 200, None, 4, 4, 0, 5)]
#[case::list_todo_with_correct_credentials("offset=1&limit=1", 200, None, 4, 1, 1, 1)]
#[case::list_todo_with_correct_credentials("offset=1&limit=2", 200, None, 4, 2, 1, 2)]
#[case::list_todo_with_correct_credentials("offset=1&limit=3", 200, None, 4, 3, 1, 3)]
#[case::list_todo_with_correct_credentials("offset=1&limit=4", 200, None, 4, 3, 1, 4)]
#[case::list_todo_with_correct_credentials("offset=2&limit=1", 200, None, 4, 1, 2, 1)]
#[case::list_todo_with_correct_credentials("offset=2&limit=2", 200, None, 4, 2, 2, 2)]
#[case::list_todo_with_correct_credentials("offset=2&limit=3", 200, None, 4, 2, 2, 3)]
#[case::list_todo_with_correct_credentials("offset=3&limit=1", 200, None, 4, 1, 3, 1)]
#[case::list_todo_with_correct_credentials("offset=3&limit=2", 200, None, 4, 1, 3, 2)]
#[case::list_todo_with_correct_credentials("offset=4&limit=1", 200, None, 4, 0, 4, 1)]
#[case::list_todo_with_correct_credentials("offset=5&limit=1", 200, None, 4, 0, 5, 1)]
#[case::list_todo_with_correct_credentials("status=completed", 200, Some("completed"), 1, 1, 0, 10)]
#[case::list_todo_with_correct_credentials("status=completed&offset=1",200,Some("completed"),1,0,1,10)]
#[case::list_todo_with_correct_credentials("status=pending", 200, Some("pending"), 1, 1, 0, 10)]
#[case::list_todo_with_correct_credentials("status=pending&offset=1",200,Some("pending"),1,0,1,10)]
#[case::list_todo_with_correct_credentials("status=progress", 200, Some("progress"), 1, 1, 0, 10)]
#[case::list_todo_with_correct_credentials("status=progress&offset=1",200,Some("progress"),1,0,1,10)]
// The test function will not see the metadata if the status not `OK 200`
#[case::list_todo_with_bad_status("status=bad", 400, None, 0, 0, 0, 0)]
#[case::list_todo_with_bad_offset("offset=bad", 400, None, 0, 0, 0, 0)]
#[case::list_todo_with_bad_limit("limit=bad", 400, None, 0, 0, 0, 0)]
#[case::list_todo_with_bad_offset_and_limit("offset=bad&limit=bad", 400, None, 0, 0, 0, 0)]
#[case::list_todo_with_bad_offset_and_correct_limit("offset=bad&limit=1", 400, None, 0, 0, 0, 0)]
#[actix_web::test]
#[serial_test::serial]
async fn list_todo_endpoint(
    #[case] params: &str,
    #[case] status_code: u16,
    #[case] status: Option<&str>,
    #[case] total: u64,
    #[case] count: u64,
    #[case] offset: u64,
    #[case] limit: u64,
) {
    let mut list_todo_res = list_todo_req(params).await;
    if status_code == 200 {
        let body = list_todo_res.body().await.unwrap();
        let body: TodoListSchema = serde_json::from_slice(body.to_vec().as_slice()).unwrap();
        assert_eq!(
            body.meta.status,
            status.map(|s| Status::from_str(s).unwrap())
        );
        assert_eq!(body.meta.total, total);
        assert_eq!(body.meta.count, count);
        assert_eq!(body.meta.offset, offset);
        assert_eq!(body.meta.limit, limit);
    }
    check_content_type(&list_todo_res);
    check_content_length(&list_todo_res);
    assert_eq!(list_todo_res.status(), status_code);
}

#[rstest::rstest]
#[case::newer_order("newer")]
#[case::older_order("older")]
#[actix_web::test]
#[serial_test::serial]
async fn list_todo_endpoint_ordring(#[case] order: &str) {
    let mut list_todo_res = list_todo_req(&format!("order={order}")).await;
    check_content_length(&list_todo_res);
    check_content_type(&list_todo_res);
    assert_eq!(list_todo_res.status(), 200);
    let body = list_todo_res.body().await.unwrap();
    let todos = serde_json::from_slice::<TodoListSchema>(body.to_vec().as_slice()).unwrap();
    let mut todos = todos.data.iter();
    let todo = todos.next().unwrap();
    let ordering = if order == "newer" {
        Ordering::Greater
    } else {
        Ordering::Less
    };

    assert!(todos.all(|t| todo.created_at.cmp(&t.created_at) == ordering));
}
