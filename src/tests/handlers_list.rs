use crate::handlers::list::get_prompt_list;
use crate::service::prompt_type::PublicPromptItem;
use actix_web::{test, web, App};

#[actix_web::test]
async fn test_get_api_list() {
    let app = test::init_service(App::new().service(get_prompt_list)).await;
    let req = test::TestRequest::get()
        .uri("/api/v1/public/prompts")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let result: Vec<PublicPromptItem> = test::read_body_json(resp).await;
    assert_eq!(result.len(), 0);
}
