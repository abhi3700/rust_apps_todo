use crate::models::Task;
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::Mutex;

// Create task
pub async fn create_task(
    task: web::Json<Task>,
    task_map: web::Data<Mutex<HashMap<u64, Task>>>,
) -> impl Responder {
    let mut map: std::sync::MutexGuard<HashMap<u64, Task>> = task_map.lock().unwrap();
    let id = map.len() as u64 + 1;
    map.insert(id, task.clone());
    HttpResponse::Created().json(id)
}

// Read tasks
pub async fn get_tasks(task_map: web::Data<Mutex<HashMap<u64, Task>>>) -> impl Responder {
    let map = task_map.lock().unwrap();
    HttpResponse::Ok().json(map.clone())
}

// Update task
pub async fn update_task(
    path: web::Path<u64>,
    task: web::Json<Task>,
    task_map: web::Data<Mutex<HashMap<u64, Task>>>,
) -> impl Responder {
    let id = path.into_inner();
    let mut map = task_map.lock().unwrap();
    map.insert(id, task.clone());
    let response = map
        .get(&id)
        .map(|_| HttpResponse::Ok().json(id))
        .unwrap_or_else(|| HttpResponse::NotFound().finish());
    response
}

// Delete task
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
