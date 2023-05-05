#![allow(dead_code)]
#![allow(unused_variables)]
mod handlers;
mod models;
mod utils;

use actix_web::{web, App, HttpServer};
use models::Task;
use std::collections::HashMap;
use std::sync::Mutex;

use handlers::{create_task, delete_task, get_tasks, update_task};

// at "/" GET API endpoint
async fn index() -> &'static str {
    "Todo App!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let task_map: web::Data<Mutex<HashMap<u64, Task>>> =
        web::Data::new(Mutex::new(HashMap::<u64, Task>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(task_map.clone())
            .service(web::resource("/index").route(web::get().to(index)))
            .service(
                web::resource("/tasks")
                    .route(web::get().to(get_tasks))
                    .route(web::post().to(create_task)),
            )
            .service(web::resource("/tasks/{id}").route(web::put().to(update_task)))
            .service(web::resource("/tasks/{id}").route(web::delete().to(delete_task)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
