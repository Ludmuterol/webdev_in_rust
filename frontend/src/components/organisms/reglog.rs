use stylist::yew::styled_component;
use yew::prelude::*;
use gloo::net::http::Request;
use gloo::console::log;

use common::LoginData;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::molecules::login_form::LoginForm;
use crate::components::molecules::register_form::RegisterForm;

#[derive(PartialEq, Clone)]
enum State {
    Login,
    Register,
}

#[styled_component(RegLog)]
pub fn reglog() -> Html {
    let on_login_submit = |data: LoginData|{
        wasm_bindgen_futures::spawn_local(async move {
            let res = Request::post("/api/login")
                .body(data.to_str().unwrap())
                .unwrap()
                .send()
                .await;
            let content = res.unwrap().text().await.unwrap();
            log!(content);
        });
    };
    let on_register_submit = |data: LoginData|{
        wasm_bindgen_futures::spawn_local(async move {
            let res = Request::post("/api/register")
                .body(data.to_str().unwrap())
                .unwrap()
                .send()
                .await;
            let content = res.unwrap().text().await.unwrap();
            log!(content);
        });
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
            background: black;
        }}>
        <div class={css!{
            display: flex;
            justify-content: center;
        }}>
            <CustomButton label="Register" handle_onclick={on_change_reg}/>
            <CustomButton label="Login" handle_onclick={on_change_log}/>
        </div>
            <br/>
        <div class={css!{
            display: flex;
            justify-content: center;
        }}>
        if *site_state == State::Login {
            <LoginForm handle_submit={on_login_submit}/>
        } else {
            <RegisterForm handle_submit={on_register_submit}/>
        }
        </div>
        </div>
    }
}
