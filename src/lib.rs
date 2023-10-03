use std::env::{self};
use log::{info};

#[derive(Debug)]
pub struct ToDo {
    pub todo: Vec<String>,
    pub file_path: String,
}

impl ToDo {
    pub fn new() -> ToDo {
        let file_path: String = env::var("TODO_PATH").expect("Expected TODO_PATH to be set as env variable.");
        info!("Creating a new ToDo file at path: {} with empty list of todos", file_path);
        ToDo { todo: vec![], file_path }
    }
}
