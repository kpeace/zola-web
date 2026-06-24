use yew::prelude::*;
use crate::components::comment_input::CommentInput;
use crate::components::comment_tree::CommentTree;
use crate::services::blog_context::BlogContext;
use crate::services::comment::CommentNode;
use crate::services::comment::build_tree;
use gloo_console::log;

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

    {
        let ctx = ctx.clone();
        let comments = comments.clone();
        let post_id = props.post_id.clone();
        let post_author_id = props.post_author_id.clone();

        use_effect_with(
            (post_id.clone(), post_author_id.clone(), comments.clone()),
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let comments_vec =
                        ctx.get_post_comments(&post_id, &post_author_id).await;

                    log!(format!("Found {} comments for post {}", comments_vec.len(), &post_id));

                    let comment_tree = build_tree(comments_vec);
                    // Debug
                    log!(format!("length of root trees vector of post comments: {}", comment_tree.len()));
                    comments.set(comment_tree);
                });

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
            />

             if !(*comments).is_empty() {
                <CommentTree
                    nodes={(*comments).clone()}
                    post_id={props.post_id.clone()}
                />
            }
        </div>
    }
}