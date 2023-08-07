use common::ProfileData;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

mod components;

use components::organisms::{reglog::RegLog, mini_profile::MiniProfile};
use components::atoms::main_title::MainTitle;

const STYLE_FILE: &str = include_str!("main.css");


#[styled_component]
pub fn App() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let profile_data_handle: UseStateHandle<Option<ProfileData>> = use_state(|| None);
    
    html! {
        <div class={stylesheet}>
            <MainTitle title="Hi there!2" />
            <RegLog profile_data_handle={profile_data_handle.clone()} />
            <MiniProfile profile_data_handle={profile_data_handle} />
        </div>
    }
}
