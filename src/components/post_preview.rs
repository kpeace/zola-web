use yew::prelude::*;
use yew_router::prelude::*;
use crate::services::blog_post::BlogPost;
use crate::router::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct PostPreviewProps {
    pub post: BlogPost,
}

#[function_component(PostPreview)]
pub fn post_preview(props: &PostPreviewProps) -> Html {

    let post = &props.post;

    html! {
        <div class="post-preview">

            <Link<Route> to={Route::Post { slug: (&props).post.slug.clone() }}>
                <h2>{ (&props).post.title.clone() }</h2>
            </Link<Route>>

            <p>{ &post.excerpt }</p>

        </div>
    }
}