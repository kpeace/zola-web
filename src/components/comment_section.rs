use yew::prelude::*;
use crate::components::comment_input::CommentInput;
use crate::components::comment_tree::CommentTree;
use crate::services::blog_context::BlogContext;
use crate::services::comment::CommentNode;
use crate::services::comment::build_tree;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post_id: String,
    pub post_author_id: String,
    pub parent_id: String,
}

#[function_component(CommentSection)]
pub fn comment_section(props: &Props) -> Html {
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");
    let comments = use_state(|| Vec::<CommentNode>::new());

    let reload_comments = {
        let ctx = ctx.clone();
        let comments = comments.clone();
        let post_id = props.post_id.clone();
        let post_author_id = props.post_author_id.clone();

        Callback::from(move |_| {
            let ctx = ctx.clone();
            let comments = comments.clone();
            let post_id = post_id.clone();
            let post_author_id = post_author_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let comments_vec =
                    ctx.get_post_comments(&post_id, &post_author_id).await;

                comments.set(build_tree(comments_vec));
            });
        })
    };

    {
        let reload_comments = reload_comments.clone();

        use_effect_with(
            (props.post_id.clone(), props.post_author_id.clone()),
            move |_| {
                reload_comments.emit(());
                || ()
            },
        );
    }

    html! {
        <div class="comments">
            <h2>{ "Comments" }</h2>

            <CommentInput
                post_id={props.post_id.clone()}
                post_author_id={props.post_author_id.clone()}
                parent_id={props.parent_id.clone()}
                parent_author_id={props.post_author_id.clone()}
                on_comment_sent={reload_comments.clone()}
            />

             if !(*comments).is_empty() {
                <CommentTree
                    nodes={(*comments).clone()}
                    post_id={props.post_id.clone()}
                    on_comment_sent={reload_comments.clone()}
                />
            }
        </div>
    }
}