use yew::prelude::*;
use crate::services::comment::CommentNode;
use crate::components::comment_input::CommentInput;
use crate::components::author_profile::AuthorProfile;

#[derive(Properties, PartialEq)]
pub struct NodeProps {
    pub node: CommentNode,
    pub post_id: String,
    pub on_comment_sent: Callback<()>,
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
                    <AuthorProfile
                        pub_key={props.node.comment.comment_author_pub_key.clone()}
                    />
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
                        post_author_pub_key={props.node.comment.root_author_pub_key.clone()}
                        parent_id={props.node.comment.id.clone()}
                        parent_author_pub_key={props.node.comment.parent_author_pub_key.clone()}
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