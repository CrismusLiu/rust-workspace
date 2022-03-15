use dioxus::prelude::*;
use tracing::info;
use crate::Todos;

#[derive(Props)]
pub struct TodoItemProps<'a> {
    pub id: u32,
    pub set_todos: &'a UseState<Todos>
}

pub fn todo_item<'a>(cx: Scope<'a, TodoItemProps<'a>>) -> Element {
    let set_edited = use_state(&cx, || false);
    let is_edited = set_edited.get();

    let TodoItemProps { id, set_todos } = cx.props;

    let todos = &set_todos.current();
    let todo = &todos[id];

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
                        info!("todo item toggled: {e:?}");
                        let mut todos = set_todos.make_mut();
                        todos.get_mut(&id).map(|todo| {
                            todo.completed = e.value.parse().unwrap();
                        });
                        todos.save();
                    }
                },
                label {
                    r#for: "todo-{todo.id}",
                    onclick: move |e| {
                        info!("label doubleclick: {e:?}");
                        set_edited.set(true);
                    },
                    "{todo.title}"
                },
            }
            is_edited.then(|| rsx! {
                input {
                    class: "edit",
                    value: "{todo.title}",
                    oninput: move |e| {
                        info!("label-editable: {e:?}");
                        let mut todos = set_todos.make_mut();
                        todos.get_mut(&id).map(|todo|{
                            todo.title = e.value.clone();
                        });
                    },
                    autofocus: "true",
                    onkeydown: move |e| {
                        match e.key.as_str() {
                            "Enter" | "Escape" | "Tab" => {
                                set_edited.set(false);
                                set_todos.make_mut().save();
                            },
                            _ => {}
                        }
                    }
                }
            })
        }
        
    }
}
