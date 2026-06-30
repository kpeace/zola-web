use yew::prelude::*;
use crate::services::{blog_context::BlogContext, comment::Comment};
use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub post_id: String,
    pub post_author_id: String,
    pub parent_id: String,
    pub parent_author_id: String,
}


#[function_component(CommentInput)]
pub fn comment_input(props: &InputProps) -> Html {
    let text = use_state(|| "".to_string());
    let sending = use_state(|| false);
    let error = use_state(|| None::<String>);
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
        let post_author_id = props.post_author_id.clone();
        let parent_id = props.parent_id.clone();
        let parent_author_id = props.parent_author_id.clone();

        let sending = sending.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let content = (*text_for_click).trim().to_string();
            //DEBUG
            log!(format!("send was clicked, content: {};; post_id {}: ;; parent_id: {}",
                content, post_id, parent_id));

            if content.is_empty() {
                return;
            }

            // Clone the values *before* moving into async
            let text_for_clear = text_for_click.clone();
            let content_clone = content.clone();
            let post_id_clone = post_id.clone();
            let parent_id_clone = parent_id.clone();
            let parent_author_id_clone = parent_author_id.clone();
            let post_author_clone = post_author_id.clone();
            let sending = sending.clone();
            let error = error.clone();
            let cloned_ctx = ctx.clone();

            let comment = Comment {
            parent_id: parent_id_clone,
            parent_author_id: parent_author_id_clone,
            root_slug: post_id_clone,
            root_author_id: post_author_clone,
            content: content_clone,
            ..Default::default()   // or manually fill other fields
        };

            wasm_bindgen_futures::spawn_local(async move {
                sending.set(true);
                if let Err(err) = cloned_ctx.client.publish_reply(comment.clone()).await {
                    log!(format!("Error sending event: {}", err));
                    error.set(Some("Failed to send reply!".to_string()));
                } else {
                    log!(format!("Comment publishd succefully: {}", comment));
                    error.set(None);
                    text_for_clear.set(String::new());
                    // reload comments
                }
                sending.set(false);
            });
        })
    };

    html! {
        <div class="comment-input">
            <div>
            <textarea
                value={(*text).clone()}
                oninput={oninput}
                disabled={*sending}
            />
            </div>
            if let Some(msg) = &*error {
                <div class="error">
                    {msg}
                </div>
            }
            <div>
            <button
                onclick={onclick}
                disabled={*sending}
            >
                {
                    if *sending {
                        "Sending..."
                    } else {
                        "Send"
                    }
                }
            </button>
            </div>
        </div>
    }
}