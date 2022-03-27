use dioxus::prelude::*;

use crate::Filter;
use crate::models::Todos;

pub fn todo_filter(cx: Scope) -> Element {
    let set_todos = use_context::<Todos>(&cx)?;
    let todos = set_todos.read();
    
    let filter = use_context::<Filter>(&cx)?;
    let fd = filter.read();
    let f = fd.to_owned();
    
    let filter = fd.to_owned();
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

    let show_clear_completed = set_todos.read().show_clear_completed();

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
                    rsx!{cx, filter_item(item: Filter::All)}
                    rsx!{cx, filter_item(item: Filter::Completed)}
                    rsx!{cx, filter_item(item: Filter::Completed)}                    
                }
                show_clear_completed.then(|| rsx! {                    
                    button {
                        class: "clear-completed",
                        onclick: move |_|  {
                            set_todos.write().clear_completed();
                        },
                        "clear completed"
                    },
                })
            }
        })
    )    
}

#[derive(Props, PartialEq)]
struct FilterItemProps {
    pub item: Filter,
}

fn filter_item(cx: Scope<FilterItemProps>) -> Element {
    let item = cx.props.item;
    let filter = use_context::<Filter>(&cx)?;

    let class = if *filter.read() == item {
        "selected"
    } else {
        ""
    };

    let onclick = move |_|  *filter.write() = item ;

    #[cfg(feature = "web")]
    {
        let href = match item {
            Filter::All => "#/",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        };
    

        rsx! {cx,
            li {
                a { class: "{ class }", href: "{href}", onclick: onclick, "{item}" },
            }
        }
    }
    
    #[cfg(feature = "desktop")]
    rsx! {
        cx,
        li {                        
            a {
                class: "{class}",
                onclick: onclick,
                "{item}"
            },
        }
    }   
    
}












