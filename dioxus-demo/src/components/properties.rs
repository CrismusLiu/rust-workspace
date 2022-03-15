use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct VoteButtonProps {
    score: i32
}

#[derive(Props, PartialEq)]
pub struct TitleCardProps<'a> {
    title: &'a str,

    name: String,
    age: i32,

    #[props(optional)]      // 可被省略
    description: Option<String>
}

#[derive(Props, PartialEq)]
pub struct TitleCardPropsOmmitDesc {
    name: String,
    age: i32,

    #[props(optional)]      // 可被省略
    description: Option<String>
}


pub fn vote_button(cx: Scope<VoteButtonProps>) -> Element {
    cx.render(rsx!(
        div {
            div {"+"}
            div {"{cx.props.score}"}
            div {"-"}            
        }
    ))
}

pub fn title_card<'a>(cx: Scope<'a, TitleCardProps<'a>>) -> Element {
    cx.render(rsx!(
        h1 { "{cx.props.title}" }
    ))
}

pub fn title_ommit_desc(cx: TitleCardPropsOmmitDesc) -> Element<'static> {    
    todo!()
}



