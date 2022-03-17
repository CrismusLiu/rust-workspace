// use serde::{Deserialize, Serialize};
// use dioxus::prelude::*;
// use std::collections::HashMap;
use dioxus_demo::app;

fn main() {
    start();
    // console_error_panic_hook::set_once();
    // tracing_wasm::set_as_global_default();
    // dioxus::web::launch(app);
}

#[cfg(feature = "desktop")]
pub fn start() {
    tracing_subscriber::fmt::init();
    dioxus::desktop::launch(app);
}

#[cfg(feature = "web")]
pub fn start() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}


// #[derive(Serialize, Deserialize)]
// pub struct DogList {
//     message: HashMap<String, Vec<String>>,
//     status: String,
// }


// fn app1(cx: Scope) -> Element {

//     let fut = 
//         use_future(&cx, (), |_| async move {
//             reqwest::get("https://dog.ceo/api/breeds/list/all")
//                 .await
//                 .unwrap()
//                 .json::<DogList>()
//                 .await
//                 .unwrap()
//         });

//     cx.render(rsx! {
//         if let Some(breeds) = fut.value() {
//             rsx!(cx, div {
//                 ul {
//                     {breeds.message.iter().map(|(breed, subbreeds)| rsx!(
//                         li {
//                             key: "{breed}",
//                             "{breed}"
//                             ul {
//                                 {subbreeds.iter().map(|subbreed| rsx!( li {
//                                     key: "{subbreed}",
//                                     "--- {subbreed}"
//                                 }))}
//                             }
//                         }
//                     ))}
//                 }
//             })
//         } else {
//             rsx!(cx, "no dogs")
//         }
//     })
// }



