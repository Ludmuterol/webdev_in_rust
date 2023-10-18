use gloo::net::http::Request;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::molecules::new_review_form::NewReviewForm;

use common::ReviewData;

#[derive(PartialEq, Clone)]
enum State {
    QueryList,
    NewReview,
}

#[styled_component(ContentMenu)]
pub fn content_menu() -> Html {
    let site_state = use_state(|| State::QueryList);
    let on_change_new = {
        let site_state = site_state.clone();
        move |_| {
            if *site_state != State::NewReview {
                site_state.set(State::NewReview);
            }
        }
    };
    let on_change_que = {
        let site_state = site_state.clone();
        move |_| {
            if *site_state != State::QueryList{
                site_state.set(State::QueryList);
            }
        }
    };
    let on_review_submit = {
        move |data: ReviewData| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::post("/api/review")
                    .body(data.to_str().unwrap())
                    .unwrap()
                    .send()
                    .await;
                let content = res.unwrap().text().await.unwrap();
            });
        }
    };
    html!{
        <>
        <div class={css!(r#"
            padding: 1em;
            height: 100%;"#
        )}>
            <div class={css!(r#"
                display:flex;
                flex-direction:row;
                "#
            )}>
                <div class={css!(r#"
                    flex: 1;"#
                )}>
                    <CustomButton label="QueryList" handle_onclick={on_change_que}/>
                </div>
                <div class={css!(r#"
                "#)}>
                    <CustomButton label="NewReview" handle_onclick={on_change_new}/>
                </div>
            </div>
        <div class={css!(r#"
        background: gray;"#
        )}>
            if *site_state == State::QueryList {
                //<LoginForm handle_submit={on_login_submit}/>
                {"QueryList"}
            } else if *site_state == State::NewReview {
                //<RegisterForm handle_submit={on_register_submit}/>
                <NewReviewForm handle_submit={on_review_submit}/>
            }
        </div>
        </div>
        </>
    }
}
