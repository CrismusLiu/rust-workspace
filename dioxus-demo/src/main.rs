use dioxus_demo::app;
// use dioxus_demo::vote_button_app;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
    // dioxus::desktop::launch(vote_button_app);

}
