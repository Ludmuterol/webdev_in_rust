use stylist::yew::styled_component;
use yew::prelude::*;

use common::ReviewData;

use crate::components::atoms::{text_input::TextInput, custom_form_button::CustomFormButton, range_input::RangeInput};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_submit: Callback<ReviewData>,
}

#[styled_component(NewReviewForm)]
pub fn new_review_form(props: &Props) -> Html {
    let url_state = use_state(|| "".to_string());
    let comment_state = use_state(|| "".to_string());
    let score_state = use_state(|| "0".to_string());
    let on_submit = Callback::from({
        let url_state = url_state.clone();
        let comment_state = comment_state.clone();
        let score_state = score_state.clone();
        let submit_handler = props.handle_submit.clone();
        move |event: SubmitEvent| {
            event.prevent_default();
            submit_handler.emit(ReviewData {
                username: "is this needed?".to_owned(),
                website: (*url_state).clone(),
                comment: (*comment_state).clone(),
                score: (*score_state).clone(),
            });
        }
    });

    let url_changed = {
        let url_state = url_state.clone();
        move |url| {
            url_state.set(url);
        }
    };
    let comment_changed = {
        let comment_state = comment_state.clone();
        move |comment| {
            comment_state.set(comment);
        }
    };
    let score_changed = Callback::from({
        let score_state = score_state.clone();
        move |score| {
            score_state.set(score);
        }
    });

    html!{
        <form onsubmit={on_submit}>
            <label>
                {"Url of Website"}
                <TextInput name="url" handle_onchange={url_changed} /> <br/>
            </label>
            <label>
                {"Comment"}
                <TextInput name="comment" handle_onchange={comment_changed} /> <br/>
            </label>
            <label>
                {"Score"}
                <RangeInput name="score" min=1 max=100 handle_onchange={score_changed} />
            </label>
            <label>
                {"Submit Button ->"}
                <CustomFormButton label="Submit" />
            </label>
        </form>
    }
}
