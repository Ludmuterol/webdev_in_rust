use common::ProfileData;
use stylist::yew::{styled_component, use_media_query};
use yew::prelude::*;

mod components;

use components::atoms::main_title::MainTitle;
use components::organisms::{mini_profile::MiniProfile, reglog::RegLog, content_menu::ContentMenu};

#[styled_component]
pub fn App() -> Html {
    let profile_data_handle: UseStateHandle<Option<ProfileData>> = use_state(|| None);
    let is_big = use_media_query("(min-width: 30em)");
    html! {
        <>
        <div class={
            if is_big {
                css!(r#"
	                display: grid;
	                gap: 10px;
                    height: 100%;
		            grid-template-columns: 5fr 1fr;
		            grid-template-rows: auto 1fr 100px;"#)
            } else {
                css!(r#"
	                display: grid;
	                gap: 10px;
                    height: 100%;"#)
            }
        }>
            <div class={css!(r#"
                grid-column: 1 / -1;
	            background: #F1F3F4;
                border-color: #d5d5d5;
                display: grid;
                color: aqua;
                display:flex;
                flex-direction:row;
            "#)}>

                <div class={css!(r#"
                    flex: 2;
                "#)}>
                    <MainTitle title="Hi there!" />
                </div>
                <div class={css!(r#"
                "#)}>
                    <MiniProfile profile_data_handle={profile_data_handle.clone()} />
                </div>
            </div>
            <div class={css!(r#"
	            background: #ffede0;
	            border-color: #df6c20;
            "#)}>
                <ContentMenu />
            </div>
            <div class={css!(r#"
	            background: #ebf5d7;
	            border-color: #8db243;
            "#)}>
                <RegLog profile_data_handle={profile_data_handle.clone()} />
            </div>
            <div class={css!(r#"
	            grid-column: 1 / -1;
	            background: #e4ebf2;
	            border-color: #8a9da8;
            "#)}>
                {"FOOTER TEXT BLABLABLA"}
            </div>
        </div>
        </>
    }
}
