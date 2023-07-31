use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub handle_onclick: Callback<()>,
}

#[styled_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    let handle_onclick = props.handle_onclick.clone();
    let onclick = Callback::from(move |_| {
        handle_onclick.emit(());
    });
    html! {
        <button class={css!{
            background-color: #008CBA;
            padding: 10px 24px;
            border-radius: 30px;
            transition-duration: 0.4s;
            :hover {
                background-color: #F44336;
            }
        }} onclick={onclick}>{&props.label}</button>
    }
}
