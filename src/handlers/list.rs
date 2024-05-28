use crate::service::promptType::PublicPromptItem;
use actix_web::{get, web, Responder, Result};

#[get("/api/v1/public/prompts")]
pub async fn get_prompt_list() -> Result<impl Responder> {
    let result: Vec<PublicPromptItem> = vec![];

    Ok(web::Json(result))
}
