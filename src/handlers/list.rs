use crate::service::prompt_type::PublicPromptItem;
use actix_web::{get, web, Responder, Result};

#[get("/api/v1/public/prompts")]
pub async fn get_prompt_list() -> Result<impl Responder> {
    let result: Vec<PublicPromptItem> = vec![];

    Ok(web::Json(result))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_sqrt() -> Result<(), String> {
//         let x = 4.0;
//         assert_eq!(2.0, x);
//         Ok(())
//     }
// }
