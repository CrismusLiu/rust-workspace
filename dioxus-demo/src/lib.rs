mod components;
mod platform;
mod models;

use dioxus::prelude::*;
use tracing::info;

use components::{ todo_filter, todo_input, todo_item};
use crate::{platform::get_store, models::Todos};
use crate::platform::Store;

use models::Filter;


pub fn app(cx: Scope) -> Element {
    // let set_todos = use_state(&cx, || {
    //     let store = get_store();
    //     store.get()
    // });
    // let todos = set_todos.get();

    // let set_filter = use_state(&cx, Filter::default);
    // let filter = set_filter.get();

    // let filter_todos = todos.iter().filter(|(_, todo)| match filter {
    //     Filter::All => true,
    //     Filter::Active => !todo.completed,
    //     Filter::Completed => todo.completed
    // })
    // .map(|(id, _)| *id)
    // .collect::<Vec<_>>();

    use_context_provider(&cx, || {
        let store = get_store();
        store.get()
    });

    use_context_provider(&cx, Filter::default);

    let todos = use_context::<Todos>(&cx)?;
    let filter = use_context::<Filter>(&cx)?;

    let filter_todos = todos.read().get_filtered_todos(&filter.read());

    info!("filter todos: {filter_todos:?}");
    
    cx.render( rsx!(
        section {
            class: "todoapp",
            style { [include_str!("style.css")] },
            div {
                rsx!(todo_input())
                ul {
                    class: "todo-list",
                    filter_todos.iter().map(|id| {
                        rsx!(todo_item(key: "{id}", id: *id))
                    })
                }
                rsx!(todo_filter())
            }
        }
    ))
}





