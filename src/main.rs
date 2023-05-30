#![allow(dead_code)]
#![allow(unused_variables)]
mod handlers;
mod models;
mod utils;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::{create_task, delete_task, get_tasks, update_task};
use models::Task;
use std::collections::HashMap;
use std::sync::Mutex;

// at "/" GET API endpoint
async fn home() -> &'static str {
    "Todo App!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_address = std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set");

    let task_map: web::Data<Mutex<HashMap<u64, Task>>> =
        web::Data::new(Mutex::new(HashMap::<u64, Task>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(task_map.clone())
            .route("/index", web::get().to(home))
            .service(
                web::resource("/tasks")
                    .route(web::get().to(get_tasks))
                    .route(web::post().to(create_task)),
            )
            .service(
                web::resource("/tasks/{id}")
                    .route(web::put().to(update_task))
                    .route(web::delete().to(delete_task)),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
