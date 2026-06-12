use yew::prelude::*;
use crate::components::comment_input::CommentInput;
use crate::components::comment_tree::CommentTree;
use crate::services::comment::build_tree;
use crate::services::comment::Comment;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post_id: String,
    pub parent_id: String,
}

#[function_component(CommentSection)]
pub fn comment_section(props: &Props) -> Html {
    let comments = use_state(|| Vec::<Comment>::new());

    // TODO: load comments via nostr here

    let tree = build_tree((*comments).clone());

    html! {
        <div class="comments">
            <h2>{ "Comments" }</h2>

            <CommentInput
                post_id={props.post_id.clone()}
                parent_id={props.parent_id.clone()}
            />

            <CommentTree nodes={tree} post_id={props.post_id.clone()} />
        </div>
    }
}