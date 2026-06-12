use yew::prelude::*;
use crate::services::{blog_context::BlogContext, comment::Comment};
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;


#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub post_id: String,
    pub parent_id: String,
}


#[function_component(CommentInput)]
pub fn comment_input(props: &InputProps) -> Html {
    let text = use_state(|| "".to_string());
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");

    let oninput = {
        let text = text.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            text.set(input.value());
        })
    };

    let text_for_click = text.clone();

    let onclick = {
        //let content = text.clone().trim().to_string();
        let post_id = props.post_id.clone();
        let parent_id = props.parent_id.clone();




        Callback::from(move |_| {
            let content = (*text_for_click).trim().to_string();
            //DEBUG
            log!(format!("send was clicked, content: {};; post_id {}: ;; parent_id: {}",
                content, post_id, parent_id));

            if content.is_empty() {
                return;
            }

            // Clone the values *before* moving into async
            let content_clone = content.clone();
            let post_id_clone = post_id.clone();
            let parent_id_clone = parent_id.clone();
            let cloned_ctx = ctx.clone();

            let comment = Comment {
            parent_id: parent_id_clone,
            root_slug: post_id_clone,
            content: content_clone,
            ..Default::default()   // or manually fill other fields
        };

            wasm_bindgen_futures::spawn_local(async move {
                if let Err(err) = cloned_ctx.client.publish_reply(comment.clone()).await {
                    log!(format!("Error sending event: {}", err));
                } else {
                    log!(format!("Comment publishd succefully: {}", comment));
                }
            });
        })
    };

    html! {
        <div class="comment-input">
            <textarea
                value={(*text).clone()}
                oninput={oninput}
            />
            <button onclick={onclick}>{ "Send" }</button>
        </div>
    }
}