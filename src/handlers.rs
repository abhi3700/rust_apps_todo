use crate::models::Task;
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::utils::determine_emoji::determine_emoji;

/// Create a task
pub async fn create_task(
    task: web::Json<Task>,
    task_map: web::Data<Mutex<HashMap<u64, Task>>>,
) -> impl Responder {
    let mut map: std::sync::MutexGuard<HashMap<u64, Task>> = task_map.lock().unwrap();
    let id = map.len() as u64 + 1;

    // append the emoji to the title
    let mut task_new = task;
    task_new.title = format!("{} {}", &task_new.title, determine_emoji(&task_new.title));

    map.insert(id, task_new.clone());
    HttpResponse::Created().json(id)
}

/// Read tasks
pub async fn get_tasks(task_map: web::Data<Mutex<HashMap<u64, Task>>>) -> impl Responder {
    let map = task_map.lock().unwrap();
    HttpResponse::Ok().json(map.clone())
}

/// Update task
/// If the id exists, update the task
/// If the id does not exist, return 404
pub async fn update_task(
    path: web::Path<u64>,
    task: web::Json<Task>,
    task_map: web::Data<Mutex<HashMap<u64, Task>>>,
) -> impl Responder {
    let id = path.into_inner();
    let mut map = task_map.lock().unwrap();
    let mut task_new = task;
    task_new.title = format!("{} {}", &task_new.title, determine_emoji(&task_new.title));

    // modify if `id` found
    map.entry(id)
        .and_modify(|t| t.title = task_new.title.clone());

    let response = map
        .get(&id)
        .map(|_| HttpResponse::Ok().json(id))
        .unwrap_or_else(|| HttpResponse::NotFound().finish());
    response
}

/// NOTE: Not required for TODO App
/// Update or Create a task
/// If the id exists, update the task
/// If the id does not exist, create the task
/* pub async fn update_or_create_task(
    path: web::Path<u64>,
    task: web::Json<Task>,
    task_map: web::Data<Mutex<HashMap<u64, Task>>>,
) -> impl Responder {
    let id = path.into_inner();
    let mut map = task_map.lock().unwrap();
    let mut task = task.into_inner();
    task.title = format!("{} {}", &task.title, determine_emoji(&task.title));
    map.entry(id)
        .and_modify(|t| t.title = task.title.clone())
        .or_insert_with(|| task);
    map.get(&id)
        .map(|_| HttpResponse::Ok().json(id))
        .unwrap_or(HttpResponse::NotFound().finish())
}
 */
/// Delete a task
pub async fn delete_task(
    path: web::Path<u64>,
    task_map: web::Data<Mutex<HashMap<u64, Task>>>,
) -> impl Responder {
    let id = path.into_inner();
    let mut map: std::sync::MutexGuard<HashMap<u64, Task>> = task_map.lock().unwrap();
    if map.remove(&id).is_some() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
