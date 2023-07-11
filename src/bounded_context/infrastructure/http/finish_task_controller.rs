use actix_web::{put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::bounded_context::builders::finish_task_builder::FinishTaskBuilder;
use crate::bounded_context::application::finish_task::FinishTaskInput;

#[derive(Debug, Deserialize)]
pub struct FinishTaskRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinishTaskResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
}

#[put("/finish_task/{id}")]
async fn finish_task(request: web::Json<FinishTaskRequest>) -> impl Responder {
    println!("Finish Task");
    let mut finish_task = FinishTaskBuilder::build();

    let input = FinishTaskInput {
        id: request.id.to_string(),
    };
    let output = finish_task.execute(input);
    
    let response = FinishTaskResponse {
        id: output.id.clone(),
        title: output.title.clone(),
        description: output.description.clone(),
        status: output.status.clone(),
    };

    HttpResponse::Ok().json(response)
}