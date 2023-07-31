use yew::prelude::*;

use crate::components::atoms::custom_form_button::CustomFormButton;
use crate::components::atoms::pass_input::PassInput;
use crate::components::atoms::text_input::TextInput;

use common::LoginData;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_submit: Callback<LoginData>,
}

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
    let username_state = use_state(|| "".to_string());
    let username_changed = Callback::from({
        let username_state = username_state.clone();
        move |username| {
            username_state.set(username);
        }
    });
    let password_state = use_state(|| "".to_string());
    let password_changed = Callback::from({
        let password_state = password_state.clone();
        move |password| {
            password_state.set(password);
        }
    });
    let on_submit = Callback::from({
        let username_state = username_state.clone();
        let password_state = password_state.clone();
        let submit_handler = props.handle_submit.clone();
        move |event: SubmitEvent| {
            event.prevent_default();
            submit_handler.emit(LoginData {
                username: (*username_state).clone(),
                password: (*password_state).clone(),
            });
        }
    });
    html! {
        <form onsubmit={on_submit}>
            <TextInput name="username" handle_onchange={username_changed} /> <br/>
            <PassInput name="password" handle_onchange={password_changed} /> <br/>
            <CustomFormButton label="Login" />
        </form>
    }
}
