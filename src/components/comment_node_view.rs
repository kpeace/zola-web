use yew::prelude::*;
use crate::services::comment::CommentNode;
use crate::components::comment_input::CommentInput;

#[derive(Properties, PartialEq)]
pub struct NodeProps {
    pub node: CommentNode,
    pub post_id: String,
    pub on_comment_sent: Callback<()>,
}

fn short_pubkey(pubkey: &str) -> String {
    if pubkey.len() < 16 {
        return pubkey.to_string();
    } else {
        return format!(
            "{}...{}",
            &pubkey[..8],
            &pubkey[pubkey.len() - 8..]
        )
    }
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
            <div class="comment-body">

                <div class="comment-header">
                    <span class="comment-author">
                        { short_pubkey(&props.node.comment.comment_author) }
                    </span>
                </div>

                <div class="comment-content">
                    { &props.node.comment.content }
                </div>

                <div class="comment-actions">
                    <button onclick={toggle}>{ "Reply" }</button>
                </div>


            </div>

            <div class="comment-children">
                if *show_reply {
                    <CommentInput
                        post_id={props.post_id.clone()}
                        post_author_id={props.node.comment.root_author_id.clone()}
                        parent_id={props.node.comment.id.clone()}
                        parent_author_id={props.node.comment.parent_author_id.clone()}
                        on_comment_sent={props.on_comment_sent.clone()}
                    />
                }

                { for props.node.children.iter().map(|child| html! {
                    <CommentNodeView
                        node={child.clone()}
                        post_id={props.post_id.clone()}
                        on_comment_sent={props.on_comment_sent.clone()}
                    />
                })}
            </div>
        </div>
    }
}