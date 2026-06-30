use yew::prelude::*;
use crate::services::comment::CommentNode;
use crate::components::comment_node_view::CommentNodeView;

#[derive(Properties, PartialEq)]
pub struct TreeProps {
    pub nodes: Vec<CommentNode>,
    pub post_id: String,
    pub on_comment_sent: Callback<()>,
}

#[function_component(CommentTree)]
pub fn comment_tree(props: &TreeProps) -> Html {
    html! {
        <>
            { for props.nodes.iter().map(|node| html! {
                <CommentNodeView
                    node={node.clone()}
                    post_id={props.post_id.clone()}
                    on_comment_sent={props.on_comment_sent.clone()}
                />
            })}
        </>
    }
}