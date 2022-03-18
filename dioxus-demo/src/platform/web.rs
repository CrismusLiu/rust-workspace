use web_sys::Storage;
use std::{collections::BTreeMap, ops::Deref};
use crate::models::Todos;
use super::Store;

const TODO_KEY: &str = "todos_doixus";


pub struct LocalStorage(Storage);

pub fn get_store() -> impl Store {
    LocalStorage::default()
}

impl Default for LocalStorage {
    fn default() -> Self {
        Self (
            web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap()
                .expect("user did not allow local storage")
        )
    }
}

impl Store for LocalStorage {

    fn get(&self) -> Todos {
        // let default_todos = Todos::default();
        if let Ok(Some(content)) = &self.get_item(&TODO_KEY) {
            // serde_json::from_str(&content).unwrap_or(default_todos)
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            // default_todos
            Default::default()
        }
    }

    fn set(&self, todos: &Todos) {
        let content = serde_json::to_string(todos).unwrap();
        &self.set_item(&TODO_KEY, &content).unwrap();
    }

}

impl Deref for LocalStorage {
    type Target = Storage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}



