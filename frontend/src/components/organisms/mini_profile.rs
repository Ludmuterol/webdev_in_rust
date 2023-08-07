use std::ops::Deref;

use common::ProfileData;
use gloo::net::http::Request;
use yew::prelude::*;
use gloo::console::log;

use crate::components::atoms::custom_button::CustomButton;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_data_handle: UseStateHandle<Option<ProfileData>>,
}

#[function_component(MiniProfile)]
pub fn mini_profile(props: &Props) -> Html {
    let profile_data_handle = props.profile_data_handle.clone();
    let on_logout = move |_|{
        let profile_data_handle = profile_data_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let res = Request::get("/api/logout")
                .send()
                .await;
            let content = res.unwrap().text().await.unwrap();
            log!(content.clone());
            match content {
                i if i == "Ok" => profile_data_handle.set(None),
                _ => (),
            };
        });
    };
    html! {
        if props.profile_data_handle.is_some() {
            {props.profile_data_handle.deref().clone().unwrap().username.to_owned()}
            <CustomButton label="Logout" handle_onclick={on_logout}/>
        } else {
            {"Wanna login?"}
        }
    }
}
