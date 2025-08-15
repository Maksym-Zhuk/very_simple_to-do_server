use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use uuid::Uuid;
use crate::entities::todo::{ActiveModel as TodoModel, Entity as Todo};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonTodo {
    pub title: String,
    pub completed: bool,
}

pub async fn get_todos(db: web::Data<DatabaseConnection>) -> HttpResponse {
    let todos = Todo::find().all(db.get_ref()).await.unwrap();
    HttpResponse::Ok().json(todos)
}

pub async fn create_todo(db: web::Data<DatabaseConnection>, new_todo: web::Json<JsonTodo>) -> HttpResponse {
    let todo = TodoModel {
        id: Set(Uuid::new_v4()),
        title: Set(new_todo.title.clone()),
        completed: Set(new_todo.completed),
        ..Default::default()
    };

    let inserted = todo.insert(db.get_ref()).await.unwrap();
    HttpResponse::Ok().json(inserted)
}

pub async fn update_todo(db: web::Data<DatabaseConnection>, update: web::Json<JsonTodo>, id: web::Path<Uuid>) -> HttpResponse {
    if let Ok(Some(todo_item)) = Todo::find_by_id(*id).one(db.get_ref()).await {
            let mut todo: TodoModel = todo_item.into();
            todo.title = Set(update.title.clone());
            todo.completed = Set(update.completed);
            let res = todo.update(db.get_ref()).await.unwrap();
            return HttpResponse::Ok().json(res);
    }

    HttpResponse::NotFound().finish()
}

pub async fn delete_todo(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> HttpResponse {
    if let Ok(Some(todo_item)) = Todo::find_by_id(*id).one(db.get_ref()).await {
        let todo: TodoModel = todo_item.into();
        todo.delete(db.get_ref()).await.unwrap();
        return HttpResponse::Ok().finish();
    }

    HttpResponse::NotFound().finish()
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not found!")
}