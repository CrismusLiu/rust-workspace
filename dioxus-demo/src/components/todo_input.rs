use dioxus::prelude::*;
use crate::{Todos,TodoItem};

#[derive(Props)]
pub struct TodoInputProps<'a> {
    pub set_todos: &'a UseState<Todos>,
}

pub fn todo_input<'a>(cx: Scope<'a, TodoInputProps<'a>>) -> Element {
    let set_draft = use_state(&cx, || "".to_string());
    let draft = set_draft.get();

    let set_todos = cx.props.set_todos;
    
    rsx!{
        cx,
        header {
            class: "header",
            h1 { "todos" },
            input {
                class: "new-todo",
                placeholder: "What needs to be done?",
                value: "{draft}",
                oninput: move |e| {
                    set_draft.set(e.value.clone());
                },
                onkeydown: move |e| {
                    if e.key == "Enter" && !draft.is_empty() {

                        let mut todos = set_todos.make_mut();
                        let id = todos.next_id;
                        todos.next_id += 1;
                        todos.insert(id, TodoItem {
                            id,
                            title: draft.clone(),
                            completed: false
                        });

                        set_draft.set("".to_string());

                        todos.save();
                    }


                }
            }
        }
    }
}
