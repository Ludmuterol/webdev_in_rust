use gloo::console::log;
use gloo::net::http::Request;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

mod components;

use components::atoms::main_title::MainTitle;
use components::molecules::login_form::LoginForm;
use components::molecules::register_form::RegisterForm;

use common::LoginData;

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component]
pub fn App() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
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
    html! {
        <div class={stylesheet}>
            <MainTitle title="Hi there!2" />
            <LoginForm handle_submit={on_login_submit}/>
            <br />
            <RegisterForm handle_submit={on_register_submit}/>
        </div>
    }
}
