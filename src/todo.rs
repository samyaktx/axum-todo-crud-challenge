use axum::{http::StatusCode, response::IntoResponse, Json, extract::Query};
use serde::Deserialize;
use serde_json::json;
use std::{collections::HashMap, sync::{Arc, Mutex}};

#[derive(PartialEq, Eq, Debug, Clone)]
struct Todo {
    title: String,
    disc: String,
    done: bool
}

struct TodoLists {
    user: HashMap<String, Vec<Todo>>
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

    // fn _create_todo(&mut self, user_id: &String, title: &String, disc: &String) -> Result<&Todo, String> {
    //     if !self.user.contains_key(user_id) {
    //         let add_todo = Todo {
    //             title: title.to_owned(),
    //             disc: disc.to_owned(),
    //             done: false
    //         };
    //         let user_todos = self.user
    //             .entry(user_id.to_owned())
    //             .or_insert(vec![add_todo]);    

    //         Ok(user_todos.last().unwrap())

    //     } else if self.user.contains_key(user_id) {
    //         let add_todo = Todo {
    //             title: title.to_owned(),
    //             disc: disc.to_owned(),
    //             done: false
    //         };
    //         let user_todos = self.user
    //             .entry(user_id.to_owned())
    //             .and_modify(|x| x.push(add_todo));

    //         Ok(user_todos.or_default().last().unwrap())

    //     } else {
    //         return Err(format!("user {} already created", user_id));
    //     }
    // }

    // better way of implementation using rust idomatics
    fn create_todo_x(&mut self, user_id: &String, title: &String, disc: &String) -> Result<&Todo, String> {
        let add_todo = Todo {
            title: title.to_owned(),
            disc: disc.to_owned(),
            done: false,
        };

        let create_todo = self.user
            .entry(user_id.to_owned())
            .and_modify(|todos| todos.push(add_todo.clone()))
            .or_insert_with(|| vec![add_todo]);

        Ok(create_todo.last().unwrap())
    }

    // better way of implementation of rust idomatics
    fn get_todos(&self, user_id: &String) -> Result<&Vec<Todo>, String> {
        self.user
            .get(user_id)
            .ok_or_else(|| format!("user {} doesn't exits", user_id))
    }

    fn _update_todo(&mut self, user_id: &String, todo_id: usize, new_title: &String, disc: &String, done: bool) -> Result<(), String> {
        if self.user.contains_key(user_id) {
            let todo = self.user.get_mut(user_id).unwrap();
                for x in 0..todo.len() {
                    if x == todo_id {
                        todo[x].title = new_title.to_owned();
                        todo[x].disc = disc.to_owned();
                        todo[x].done = done;
                    } else {
                        return Err(format!("user's todo id: {} doesn't exist", todo_id));
                    }
                }
            Ok(())
        } else {
            return Err(format!("user {} doesn't exist", user_id));
        }
    }

    // better way of implementation of rust idomatics
    fn update_todo_x(&mut self, user_id: &String, todo_id: usize, new_title: &String, disc: &String, done: bool) -> Result<(), String> {
        let todos = self.user
            .get_mut(user_id)
            .ok_or_else(|| format!("user {} doesn't exist", user_id))
            .and_then(|todo| {
                if todo_id < todo.len()   {
                    todo[todo_id].title = new_title.to_owned();
                    todo[todo_id].disc = disc.to_owned();
                    todo[todo_id].done = done;
                    Ok(())
                } else {
                    Err(format!("user's todo id: {} doesn't exist", todo_id))
                }
            });

        todos
    }

    fn _delete_todo(&mut self, user_id: &String, todo_id: usize) -> Result<Todo, String> {
        if self.user.contains_key(user_id) {
            let remove_todo = self.user.get_mut(user_id).unwrap();
            if todo_id < remove_todo.len(){
                let removed = remove_todo.remove(todo_id);
                Ok(removed)
            } else {
                return  Err(format!("todo_id {} doesn't exist", todo_id));
            }
        } else {
            return  Err(format!("user {} doesn't exist", user_id));
        }
    }

    // my way - implementation of rust idomatics
    fn delete_todo_x(&mut self, user_id: &String, todo_id: usize) -> Result<Todo, String> {
        let remove_todo = self.user
            .get_mut(user_id)
            .filter(|x| x.len() > todo_id)
            .and_then(| x| Some(x.remove(todo_id)))
            .ok_or_else(|| format!("user {} doesn't exist", user_id));

        remove_todo
    }

    // grok way - implementation of rust idomatics
    fn _delete_todo_grok(&mut self, user_id: &String, todo_id: usize) -> Result<Todo, String> {
        // First, check if the user exists
        let todos = self.user.get_mut(user_id)
            .ok_or_else(|| format!("User {} does not exist", user_id))?;

        // Check if the index is within bounds
        if todo_id >= todos.len() {
            return Err(format!("Todo index {} is out of bounds", todo_id));
        }

        // Remove the todo at the given index
        Ok(todos.remove(todo_id))
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
    todo_id: usize,
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
    let created_todo = todo_guard.create_todo_x(&user_id, &title, &disc);
    
    (StatusCode::CREATED, Json(json!(format!("user {} created {:?}", user_id, created_todo))))
}

pub async fn update_todo(Json(update_todo_payload): Json<UpdateTodoPayload>) -> impl IntoResponse {
    let user_id = update_todo_payload.user_id; 
    let todo_id = update_todo_payload.todo_id; 
    let new_title = update_todo_payload.new_title; 
    let disc = update_todo_payload.new_disc; 
    let done = update_todo_payload.done; 
    let create_todo_instance = TodoLists::get_instance();
    let mut todo_guard = create_todo_instance.lock().unwrap();
    todo_guard.update_todo_x(&user_id, todo_id, &new_title, &disc, done).ok();
    
    let updated_todo = format!("updated todo: title: {}, disc: {}, done: {} ", new_title, disc, done);
    
    (StatusCode::OK, Json(json!({"updated_todo": updated_todo})))
}

#[derive(Deserialize)]
pub struct ViewAllTodos {
    user_id: String
}

pub async fn get_todo(Query(query_param): Query<ViewAllTodos>) -> impl IntoResponse {
    let user_id = query_param.user_id;
    let create_todo_instance = TodoLists::get_instance();
    let todo_guard = create_todo_instance.lock().unwrap();
    let todo = todo_guard.get_todos(&user_id);
    
    (StatusCode::OK, Json(json!(format!("user_id: {user_id}, todo: {todo:#?}"))))
}


#[derive(Deserialize)]
pub struct DeleteTodo {
    user_id: String,
    todo_id: usize,
}

pub async fn delete_todo(Json(query_param): Json<DeleteTodo>) -> impl IntoResponse {
    let user_id = query_param.user_id;
    let todo_id = query_param.todo_id;
    let create_todo_instance = TodoLists::get_instance();
    let mut todo_guard = create_todo_instance.lock().unwrap();
    let todo = todo_guard.delete_todo_x(&user_id, todo_id).unwrap();
    
    (StatusCode::OK, Json(json!(format!("user_id: {user_id}, deleted_todo: {todo:#?}"))))
}