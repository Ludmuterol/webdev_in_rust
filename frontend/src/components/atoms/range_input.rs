use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub min: u32,
    pub max: u32,
    pub handle_onchange: Callback<String>,
}

#[function_component(RangeInput)]
pub fn range_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap(); //youtube guy says he feels pretty sure that there is
                                              //alwas something there
        let input = target.unchecked_into::<HtmlInputElement>();
        handle_onchange.emit(input.value());
    });
    html! {
        <input type="range" min={props.min.to_string()} max ={props.max.to_string()} name={props.name.to_owned()} onchange={onchange} />
    }
}
