use actix_web::{put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::bounded_context::builders::start_task_builder::StartTaskBuilder;
use crate::bounded_context::application::start_task::StartTaskInput;

#[derive(Debug, Deserialize)]
pub struct StartTaskRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTaskResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
}

#[put("/task/{id}")]
async fn start_task(request: web::Json<StartTaskRequest>) -> impl Responder {
    
    let mut start_task = StartTaskBuilder::build();

    let input = StartTaskInput {
        id: request.id.to_string(),
    };
    let output = start_task.execute(input);
    
    let response = StartTaskResponse {
        id: output.id.clone(),
        title: output.title.clone(),
        description: output.description.clone(),
        status: output.status.clone(),
    };

    HttpResponse::Ok().json(response)
}