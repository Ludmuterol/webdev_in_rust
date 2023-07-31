use gloo::console::log;
use gloo::net::http::Request;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

mod components;

use components::organisms::reglog::RegLog;
use components::atoms::main_title::MainTitle;
use components::atoms::custom_button::CustomButton;

const STYLE_FILE: &str = include_str!("main.css");


#[styled_component]
pub fn App() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    
    let on_logout = |_|{
        wasm_bindgen_futures::spawn_local(async move {
            let res = Request::get("/api/logout")
                .send()
                .await;
            let content = res.unwrap().text().await.unwrap();
            log!(content);
        });
    };
    html! {
        <div class={stylesheet}>
            <MainTitle title="Hi there!2" />
            <RegLog />
            <CustomButton label="Logout" handle_onclick={on_logout}/>
        </div>
    }
}
