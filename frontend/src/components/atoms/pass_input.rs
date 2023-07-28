use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(PassInput)]
pub fn pass_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap(); //youtube guy says he feels pretty sure that there is
                                              //alwas something there
        let input = target.unchecked_into::<HtmlInputElement>();
        handle_onchange.emit(input.value());
    });
    html! {
       <input type="password" name={props.name.to_owned()} onchange={onchange} />
    }
}
