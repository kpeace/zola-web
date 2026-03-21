use yew::prelude::*;
use crate::components::post_preview::PostPreview;
use crate::services::blog_context::BlogContext;

#[function_component(PostList)]
pub fn post_list() -> Html {
    // Debug
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");

    if ctx.state.posts.as_ref().is_empty() {
        return html! {
            <div>{ "Loading posts..." }</div>
        };
    }

    html! {
        <div class="post-list">
            { for ctx.state.posts.as_ref().iter().map(|p| html!{
                <PostPreview post={p.clone()} />
            })}
        </div>
    }
}