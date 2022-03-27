use std::ops::Add;

use dioxus::prelude::*;
use tracing::info;
use crate::models::Todos;

#[derive(Props, PartialEq)]
pub struct TodoItemProps {
    pub id: u32,
}

pub fn todo_item(cx: Scope<TodoItemProps>) -> Element {
    let set_edited = use_state(&cx, || false);
    let is_edited = set_edited.get();

    let id = cx.props.id;
    let todos = use_context::<Todos>(&cx)?;
    let todo_read = todos.read();
    let todo = todo_read.get(&id)?;

    let set_draft = use_state(&cx, || todo.title.clone());
    let draft = set_draft.get();
    
    let completed = if todo.completed { "completed" } else { "" };
    let editing = if *is_edited { "editing" } else {""};    
    
    rsx!{ cx, 
        li {
            class: "{completed} {editing}",
            div {
                class: "view",
                input {
                    id: "todo-{todo.id}",
                    class: "toggle",
                    r#type: "checkbox", 
                    checked: "{todo.completed}",
                    onchange: move |e| {
                        info!("checkbox change: {e:?}");
                        todos.write().checkbox_toggle(id);
                    }
                },
                label {
                    r#for: "todo-{todo.id}",
                    onclick: move |e| {
                        info!("label click: {e:?}");
                        set_edited.set(true);
                    },
                    "{todo.title}"
                },
            }
            is_edited.then(|| rsx! {
                input {
                    class: "edit",
                    value: "{draft}",
                    oninput: move |e| {
                        info!("label-editable: {e:?}");
                        set_draft.set(e.value.clone());
                    },
                    autofocus: "true",
                    onkeydown: move |e| {
                        match e.key.as_str() {
                            "Enter" | "Escape" | "Tab" => {
                                set_edited.set(false);
                                info!("input edit draft: {draft:?}");
                                todos.write().update_todo(id, draft);
                            },
                            _ => {}
                        }
                    }
                }
            })
        }        
    }
}
