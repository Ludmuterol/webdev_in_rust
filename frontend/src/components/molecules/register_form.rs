use stylist::yew::styled_component;
use yew::prelude::*;
use zxcvbn::feedback::{Suggestion, Warning};
use zxcvbn::zxcvbn;

use crate::components::atoms::custom_form_button::CustomFormButton;
use crate::components::atoms::pass_input::PassInput;
use crate::components::atoms::text_input::TextInput;

use common::LoginData;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_submit: Callback<LoginData>,
}

#[styled_component(RegisterForm)]
pub fn register_form(props: &Props) -> Html {
    let password_estimate = use_state(|| 0u8);
    let sugtest: UseStateHandle<Option<Vec<Suggestion>>> = use_state(|| None);
    let warning = use_state(|| None::<Warning>);
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
        let username_state = username_state.clone();
        let password_estimate = password_estimate.clone();
        let sugtest = sugtest.clone();
        let warning = warning.clone();
        move |password: String| {
            if password.len() > 0 {
                let estimate = zxcvbn(password.clone().as_str(), &[&*username_state]).unwrap();
                password_estimate.set(estimate.score());
                match estimate.feedback() {
                    None => {
                        sugtest.set(None);
                        warning.set(None);
                    }
                    Some(feedback) => {
                        let sugs = feedback.suggestions();
                        match sugs.len() {
                            i if i > 0 => sugtest.set(Some(sugs.to_vec())),
                            0 => sugtest.set(None),
                            _ => unreachable!(),
                        }
                        warning.set(feedback.warning());
                    }
                };
            } else {
                sugtest.set(None);
                warning.set(None);
            }
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
        <label>
        <h2> {"Register"} </h2>
        <form onsubmit={on_submit}>
            <label>
                {"Username"}
                <TextInput name="username" handle_onchange={username_changed} /> <br/>
            </label>
            <label>
                {"Password"}
                <PassInput name="password" handle_onchange={password_changed} /> <br/>
            </label>
            {"Score: "} {*password_estimate} <br/>
            if (*warning).is_some() {
                <div class={css!(r#"
                    background: #FF7171;
                    border: solid #FF0000;
                    border-radius: 25px;
                    padding: 12.5px;
                    margin: 5px;
                "#)}>
                    {*warning}
                </div>
            }
            if (*sugtest).is_some() {
                {
                    (*sugtest).clone().unwrap().iter().map(|sug| {
                        html! {
                            <div class={css!(r#"
                                background: #FFB46B;
                                border: solid #FF7F00;
                                border-radius: 25px;
                                padding: 12.5px;
                                margin: 5px;
                            "#)}>
                                {sug.to_string()}
                            </div>
                        }
                    }).collect::<Html>()
                }
            }
            <CustomFormButton label="Register" />
        </form>
        </label>
    }
}
