use std::{io::{Read, Write}, path::PathBuf};
use crate::models::Todos;
use std::fs::File;
use std::env;
use super::Store;

const TODO_FILE: &str = "todos_doixus.json";

pub struct FileStore {
    pub path: PathBuf,
}

pub fn get_store() -> impl Store {
    FileStore::default()
}


impl Default for FileStore {
    fn default() -> Self {
        let path = env::current_dir().unwrap().join(TODO_FILE);
        Self {
            path
        }
    }
}

impl Store for FileStore {
    fn get(&self) -> Todos {
        let default_todos = Todos::default();
        if let Ok(mut file) = File::open(&self.path) {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            serde_json::from_str(&content).unwrap_or(default_todos)
        } else {
            // Default::default()
            default_todos
        }
    }

    fn set(&self, todos: &Todos) {
        let content = serde_json::to_string(todos).unwrap();
        let mut file = File::create(&self.path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

}
