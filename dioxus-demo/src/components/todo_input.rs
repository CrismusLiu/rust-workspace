use dioxus::prelude::*;
use crate::models::Todos;

pub fn todo_input(cx: Scope) -> Element {
    let set_draft = use_state(&cx, || "".to_string());
    let draft = set_draft.get();
    let todos = use_context::<Todos>(&cx)?;
    
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
                        todos.write().create_todo(draft);
                        set_draft.set("".to_string());
                    }
                }
            }
        }
    }
}
