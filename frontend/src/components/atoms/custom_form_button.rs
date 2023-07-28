use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
}

#[function_component(CustomFormButton)]
pub fn custom_form_button(props: &Props) -> Html {
    html! {
        <button>{&props.label}</button>
    }
}
