use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub handle_onclick: Callback<()>,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    let handle_onclick = props.handle_onclick.clone();
    let onclick = Callback::from(move |_|{
        handle_onclick.emit(());
    });
    html! {
        <button onclick={onclick}>{&props.label}</button>
    }
}
