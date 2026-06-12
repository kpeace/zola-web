use yew::prelude::*;
use crate::services::comment::CommentNode;
use crate::components::comment_input::CommentInput;

#[derive(Properties, PartialEq)]
pub struct NodeProps {
    pub node: CommentNode,
    pub post_id: String,
}

#[function_component(CommentNodeView)]
pub fn comment_node(props: &NodeProps) -> Html {
    let show_reply = use_state(|| false);

    let toggle = {
        let show_reply = show_reply.clone();
        Callback::from(move |_| show_reply.set(!*show_reply))
    };

    html! {
        <div class="comment">
            <p>{ &props.node.comment.content }</p>

            <button onclick={toggle}>{ "Reply" }</button>

            if *show_reply {
                <CommentInput
                    post_id={props.post_id.clone()}
                    parent_id={props.node.comment.id.clone()}
                />
            }

            <div class="children">
                { for props.node.children.iter().map(|child| html! {
                    <CommentNodeView node={child.clone()} post_id={props.post_id.clone()} />
                })}
            </div>
        </div>
    }
}