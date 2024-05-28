pub mod handlers;
pub mod service;
pub mod tests;

use actix_web::{App, HttpServer};
use handlers::list::get_prompt_list;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_prompt_list))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
