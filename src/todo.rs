use axum::{http::StatusCode, response::IntoResponse, Json, extract::Query};
use serde::Deserialize;
use serde_json::json;
use std::{borrow::BorrowMut, collections::HashMap, sync::{Arc, Mutex}};

#[derive(PartialEq, Eq, Debug)]
struct Todo {
    title: String,
    disc: String,
    done: bool
}

struct TodoLists {
    user: HashMap<String, Todo>
}


impl Todo {
    fn new() -> Self {
        Self {
            title: "".to_owned(),
            disc: "".to_owned(),
            done: false,
        }
    }
}

impl TodoLists {
    fn new() -> Self {
        Self {
            user: HashMap::new()
        }
    }

    fn get_instance() -> Arc<Mutex<TodoLists>> {
        static mut INSTANCE: Option<Arc<Mutex<TodoLists>>> = None;

        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Arc::new(Mutex::new(TodoLists::new())));
            }
            
            INSTANCE.as_ref().unwrap().clone()
        }
    }

    fn create_todo(&mut self, user_id: &String, title: &String, disc: &String) -> Result<&Todo, String> {
        if !self.user.contains_key(user_id) {
            let todo = self.user.entry(user_id.to_owned()).or_insert(Todo::new());
            todo.title = title.to_owned();
            todo.disc = disc.to_owned();
            Ok(todo)
        } else {
            return Err(format!("user {} already created", user_id));
        }
    }

    fn get_todo(&self, user_id: &String) -> Result<&Todo, String> {
        if self.user.contains_key(user_id) {
            let todos = self.user.get(user_id).unwrap();
            Ok(todos)
        } else {
            return Err(format!("user {} doesn't exist", user_id));
        }
    }
    fn update_todo(&mut self, user_id: &String, new_title: &String, disc: &String, done: bool) -> Result<(), String> {
        if self.user.contains_key(user_id) {
            let todo = self.user.get_mut(user_id).unwrap();
                todo.title = new_title.to_owned();
                todo.disc = disc.to_owned();
                todo.done = done;
            Ok(())
        } else {
            return Err(format!("user {} doesn't exist", user_id));
        }
    }

    fn delete_todo(&mut self, user_id: &String) -> Result<Todo, String> {
        if self.user.contains_key(user_id) {
            let remove_todo = self.user.borrow_mut().remove(user_id).unwrap();
            Ok(remove_todo)
        } else {
            return  Err(format!("user {} doesn't exist", user_id));
        }
    }
}

#[derive(Deserialize)]
pub struct TodoPayload {
    user_id: String,
    title: String,
    disc: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoPayload {
    user_id: String,
    new_title: String,
    new_disc: String,
    done: bool
}

pub async fn create_todo(Json(todo_payload): Json<TodoPayload>) -> impl IntoResponse {
    let user_id = todo_payload.user_id; 
    let title = todo_payload.title; 
    let disc = todo_payload.disc; 
    let create_todo_instance = TodoLists::get_instance();
    let mut todo_guard = create_todo_instance.lock().unwrap();
    let created_todo = todo_guard.create_todo(&user_id, &title, &disc);
    
    (StatusCode::CREATED, Json(json!(format!("user {} created {:?}", user_id, created_todo))))
}

pub async fn update_todo(Json(update_todo_payload): Json<UpdateTodoPayload>) -> impl IntoResponse {
    let user_id = update_todo_payload.user_id; 
    let new_title = update_todo_payload.new_title; 
    let disc = update_todo_payload.new_disc; 
    let done = update_todo_payload.done; 
    let create_todo_instance = TodoLists::get_instance();
    let mut todo_guard = create_todo_instance.lock().unwrap();
    todo_guard.update_todo(&user_id, &new_title, &disc, done).ok();
    
    let updated_todo = format!("updated todo: title: {}, disc: {}, done: {} ", new_title, disc, done);
    
    (StatusCode::OK, Json(json!({"updated_todo": updated_todo})))
}

#[derive(Deserialize)]
pub struct QueryParam {
    user_id: String
}

pub async fn get_todo(Query(query_param): Query<QueryParam>) -> impl IntoResponse {
    let user_id = query_param.user_id;
    let create_todo_instance = TodoLists::get_instance();
    let todo_guard = create_todo_instance.lock().unwrap();
    let todo = todo_guard.get_todo(&user_id);
    
    (StatusCode::OK, Json(json!(format!("user_id: {user_id}, todo: {todo:#?}"))))
}

pub async fn delete_todo(Query(query_param): Query<QueryParam>) -> impl IntoResponse {
    let user_id = query_param.user_id;
    let create_todo_instance = TodoLists::get_instance();
    let mut todo_guard = create_todo_instance.lock().unwrap();
    let todo = todo_guard.delete_todo(&user_id).unwrap();
    
    (StatusCode::OK, Json(json!(format!("user_id: {user_id}, deleted_todo: {todo:#?}"))))
}