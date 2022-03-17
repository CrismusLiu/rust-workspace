mod components;
mod platform;

use std::{collections::BTreeMap, ops::{Deref, DerefMut}};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use components::{ todo_filter, todo_input, todo_item};

use crate::platform::get_store;
use crate::platform::Store;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u32,
    pub title: String, 
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  Todos {
    todos: BTreeMap<u32, TodoItem>,
    next_id: u32
}

impl Default for Todos {
    fn default() -> Self {
        Self {
            todos: BTreeMap::new(),
            next_id: 1
        }
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



pub fn app(cx: Scope) -> Element {
    let set_todos = use_state(&cx, || {
        let store = get_store();
        store.get()
    });
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
                        rsx!(todo_item(key: "{id}", id: *id, set_todos: set_todos))
                    })
                }
                rsx!(todo_filter(set_todos: set_todos, set_filter: set_filter))
            }
        }
    ))
}





