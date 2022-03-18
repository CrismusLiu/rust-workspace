use dioxus::prelude::*;

use crate::Filter;
use crate::models::Todos;


#[derive(Props)]
pub struct TodoFilterProps<'a> {
    pub set_todos: &'a UseState<Todos>,
    pub set_filter: &'a UseState<Filter>,
}

pub fn todo_filter<'a>(cx: Scope<'a, TodoFilterProps<'a>>) -> Element {
    let set_filter = cx.props.set_filter;
    let set_todos = cx.props.set_todos;
    let todos = set_todos.get();

    let filter = set_filter.get();
    let items_left = match filter {
        Filter::All => todos.len(),
        Filter::Active  => {
            let active  = 
                 todos.iter()
                .filter(|(_, item)|  !item.completed)
                .map(|(id, _)| *id)
                .collect::<Vec<_>>();
                active.len()
        }, 
        Filter::Completed => {
            let completed  = 
                todos.iter()
                .filter(|(_, item)|  item.completed)
                .map(|(id, _)| *id)
                .collect::<Vec<_>>();
            completed.len()
        },
    };

    let item_text = if items_left == 1 {
        "item left"
    } else {
        "items left"
    };

    let show_clear_completed = set_todos.get().show_clear_completed();

    let seletc_context = |f: &Filter| if f == set_filter.get() { "selected" } else { "" };

    let selected_all_class = seletc_context(&Filter::All);
    let selected_active_class = seletc_context(&Filter::Active);
    let selected_completed_class = seletc_context(&Filter::Completed);

    rsx!(cx, 
        (!todos.is_empty()).then(|| rsx!{
            footer {
                class: "footer",
                span {
                    class: "todo-count",
                    strong { "{items_left}" },
                    span { "{item_text}" },
                },
                ul {
                    class: "filters",
                    li {                        
                        a {
                            class: "{selected_all_class}" ,
                            href: "#/all",
                            onclick: move |_| {
                                set_filter.set(Filter::All) ;
                            },
                            "All"
                        },
                    },
                    li {                        
                        a {
                            class: "{selected_active_class}",
                            href: "#/active",
                            onclick: move |_| set_filter.set(Filter::Active) ,
                            "Active"
                        },
                    },
                    li {                        
                        a {
                            class: "{selected_completed_class}",
                            href: "#/completed",
                            onclick: move |_| set_filter.set(Filter::Completed) ,
                            "Completed"
                        },
                    }
                }
                show_clear_completed.then(|| rsx! {                    
                    button {
                        class: "clear-completed",
                        onclick: move |_|  {
                            let mut todos = set_todos.make_mut();
                            todos.clear_completed();
                        },
                        "clear completed"
                    },
                })
            }
        })
    )


    
}

