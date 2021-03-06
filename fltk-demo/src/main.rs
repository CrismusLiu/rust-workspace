// use fltk::{app, prelude::*, window::Window};

/// 去黑色命令行界面
#[windows_subsystem = "windows"]
use fltk::{app, button::Button, frame::Frame, group::Pack, prelude::*, window::Window};
fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    // Vertical is default. You can choose horizontal using pack.set_type(PackType::Horizontal);
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_inc = Button::default().with_size(0, 40).with_label("+");
    let mut frame = Frame::default().with_size(0, 40).with_label("0");
    let mut but_dec = Button::default().with_size(0, 40).with_label("-");

    pack.end();
    wind.make_resizable(true);
    wind.end();
    wind.show();


    let (s, r) = app::channel::<Message>();

    but_inc.emit(s, Message::Increment);
    but_dec.emit(s, Message::Decrement);

    while app.wait() {
        let label: i32 = frame.label().parse().unwrap();

        if let Some(msg) = r.recv() {
            match msg {
                Message::Increment => frame.set_label(&(label + 1).to_string()),
                Message::Decrement => frame.set_label(&(label - 1).to_string()),
            }
        }
    }


    app.run().unwrap();
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}