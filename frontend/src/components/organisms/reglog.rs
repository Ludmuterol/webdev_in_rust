use stylist::yew::styled_component;
use yew::prelude::*;
use gloo::net::http::Request;

use common::{LoginData, ProfileData};

use crate::components::atoms::custom_button::CustomButton;
use crate::components::molecules::login_form::LoginForm;
use crate::components::molecules::register_form::RegisterForm;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_data_handle: UseStateHandle<Option<ProfileData>>,
}

#[derive(PartialEq, Clone)]
enum State {
    Login,
    Register,
}

#[styled_component(RegLog)]
pub fn reglog(props: &Props) -> Html {
    let response_string = use_state(||"".to_owned());
    let profile_data_handle = props.profile_data_handle.clone();
    let on_login_submit = {
        let response_string = response_string.clone();
        let profile_data_handle = profile_data_handle.clone();
        move |data: LoginData|{
            let response_string = response_string.clone();
            let profile_data_handle = profile_data_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::post("/api/login")
                    .body(data.to_str().unwrap())
                    .unwrap()
                    .send()
                    .await;
                let content = res.unwrap().text().await.unwrap();
                response_string.set(content.to_owned());
                match content {
                    i if i == "Ok" => {
                        let res = Request::get("/api/profile")
                            .send()
                            .await;
                        let res = ProfileData::from_str(&res.unwrap().text().await.unwrap());
                        match res {
                            Some(val) => profile_data_handle.set(Some(val)),
                            None => profile_data_handle.set(None),
                        };
                    },
                    _ => profile_data_handle.set(None),
                }
            });
        }
    };
    let on_register_submit = {
        let response_string = response_string.clone();
        move |data: LoginData|{
            let response_string = response_string.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response_string = response_string.clone();
                let res = Request::post("/api/register")
                    .body(data.to_str().unwrap())
                    .unwrap()
                    .send()
                    .await;
                let content = res.unwrap().text().await.unwrap();
                response_string.set(content);
            });
        }
    }; 
    let site_state = use_state(|| State::Login);
    let on_change_reg = {
        let site_state = site_state.clone();
        move |_| {
            if *site_state != State::Register {
                site_state.set(State::Register);
            }
        }
    };
    let on_change_log = {
        let site_state = site_state.clone();
        move |_| {
            if *site_state != State::Login {
                site_state.set(State::Login);
            }
        }
    };
    html! {
        <div class={css!{
            background: gray;
        }}>
        
            <CustomButton label="Register" handle_onclick={on_change_reg}/>
            <CustomButton label="Login" handle_onclick={on_change_log}/>
            <br/>
                if *site_state == State::Login {
            <LoginForm handle_submit={on_login_submit}/>
        } else {
            <RegisterForm handle_submit={on_register_submit}/>
        }
        <br/>
                {&*response_string}
        </div>
    }
}
