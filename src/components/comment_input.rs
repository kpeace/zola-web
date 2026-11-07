use yew::prelude::*;
use crate::services::{blog_context::BlogContext, comment::Comment};
use crate::services::public_key::PublicKey;
use gloo_console::log;
use crate::components::logedin_user::LogedinUser;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub post_id: String,
    pub post_author_pub_key: PublicKey,
    pub parent_id: String,
    pub parent_author_pub_key: PublicKey,
    pub on_comment_sent: Callback<()>,
}


#[function_component(CommentInput)]
pub fn comment_input(props: &InputProps) -> Html {
    let text = use_state(|| "".to_string());
    let sending = use_state(|| false);
    let is_loggedin = use_state(|| false);
    let error = use_state(|| None::<String>);
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");

    let ctx_chk_loggedin = ctx.clone();
    let is_loggedin_clone = is_loggedin.clone();

    // gets input into the text variable
    let oninput = {
        let text = text.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            text.set(input.value());
        })
    };

    let text_for_click = text.clone();

    // disables the "send" button after it's clicked
    let onclick = {
        //let content = text.clone().trim().to_string();
        let post_id = props.post_id.clone();
        let post_author_id = props.post_author_pub_key.clone();
        let parent_id = props.parent_id.clone();
        let parent_author_pub_key = props.parent_author_pub_key.clone();

        let sending = sending.clone();
        let error = error.clone();

        let on_comment_sent_clone = props.on_comment_sent.clone();

        Callback::from(move |_| {
            let content = (*text_for_click).trim().to_string();

            if content.is_empty() {
                return;
            }

            // Clone the values *before* moving into async
            let text_for_clear = text_for_click.clone();
            let content_clone = content.clone();
            let post_id_clone = post_id.clone();
            let parent_id_clone = parent_id.clone();
            let parent_author_pub_key_clone = parent_author_pub_key.clone();
            let post_author_clone = post_author_id.clone();
            let sending = sending.clone();
            let error = error.clone();
            let cloned_ctx = ctx.clone();
            let on_comment_sent_clone = on_comment_sent_clone.clone();

            let comment = Comment {
                parent_id: parent_id_clone,
                parent_author_pub_key: parent_author_pub_key_clone,
                root_slug: post_id_clone,
                root_author_pub_key: post_author_clone,
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
                    on_comment_sent_clone.emit(());
                }
                sending.set(false);
            });
        })
    };


    // disables send if user not logged in
    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            is_loggedin_clone.set(ctx_chk_loggedin.is_loggedin().await);
        });
    });

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
                disabled={*sending || !*is_loggedin}
            >
                {
                    if *sending {
                        "Sending..."
                    } else {
                        "Send"
                    }
                }
            </button>
            <div>
                <LogedinUser />
            </div>
            </div>
        </div>
    }
}