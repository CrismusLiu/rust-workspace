mod components;

use std::{collections::BTreeMap, hash::Hash, ops::{Deref, DerefMut}};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use components::{ todo_filter, todo_input, todo_item};
use web_sys::Storage;

const TODO_KEY: &str = "todos_doixus";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u32,
    pub title: String, 
    pub completed: bool,
}

// pub type Todos = BTreeMap<u32, TodoItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  Todos {
    todos: BTreeMap<u32, TodoItem>,
    next_id: u32
}

impl Default for Todos {
    fn default() -> Self {
        let store = get_store();
        store.get()
    }
}

impl Deref for Todos {
    type Target = BTreeMap<u32, TodoItem>;
    fn deref(&self) -> &Self::Target {
        &self.todos
    }
}

impl DerefMut for Todos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.todos
    }
}

impl Todos {
    fn save(&self) {
        let store = get_store();
        store.set(self);
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed
}

impl Default for Filter {
    fn default() -> Self { 
        let url_hash = web_sys::window().unwrap().location().hash().unwrap();
        match url_hash.as_str() {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All
        }
    }
}


pub fn app(cx: Scope) -> Element {
    let set_todos = use_state(&cx, Todos::default);
    let todos = set_todos.get();

    let set_filter = use_state(&cx, Filter::default);
    let filter = set_filter.get();

    let filter_todos = todos.iter().filter(|(_, todo)| match filter {
        Filter::All => true,
        Filter::Active => !todo.completed,
        Filter::Completed => todo.completed
    })
    .map(|(id, _)| *id)
    .collect::<Vec<_>>();


    info!("todos: {todos:?}, filter todos: {filter_todos:?}");
    
    cx.render( rsx!(
        section {
            class: "todoapp",
            style { [include_str!("style.css")] },
            div {
                rsx!(todo_input(set_todos: set_todos))
                ul {
                    class: "todo-list",
                    filter_todos.iter().map(|id| {
                        rsx!(todo_item(id: *id, set_todos: set_todos))
                    })
                }
                rsx!(todo_filter(set_todos: set_todos, set_filter: set_filter))
            }
        }
    ))
}

pub fn get_store() -> impl Store {
    LocalStorage::default()
}

pub struct LocalStorage(Storage);

pub trait Store {
    fn get(&self) -> Todos;
    fn set(&self, todos: &Todos);
}


impl Deref for LocalStorage {
    type Target = Storage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
        let default_todos = Todos {
            todos: BTreeMap::new(),
            next_id: 1
        };
        if let Ok(Some(content)) = self.get_item(&TODO_KEY) {
            serde_json::from_str(&content).unwrap_or(default_todos)
        } else {
            default_todos
        }
    }

    fn set(&self, todos: &Todos) {
        let content = serde_json::to_string(todos).unwrap();
        self.set_item(&TODO_KEY, &content).unwrap();
    }

}




