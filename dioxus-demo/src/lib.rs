mod components;

use std::collections::BTreeMap;

use dioxus::prelude::*;
use tracing::info;

use components::{ todo_filter, todo_input, todo_item};


#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub title: String, 
    pub completed: bool,
}

pub type Todos = BTreeMap<u32, TodoItem>;

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed
}

impl Default for Filter {
    fn default() -> Self { Filter::All }
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

    let left_filter = filter_todos.len();

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
                rsx!(todo_filter(items_left: left_filter, set_todos: set_todos, set_filter: set_filter))
            }
        }
    ))

}

