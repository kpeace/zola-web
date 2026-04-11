use yew::prelude::*;
use yew_router::prelude::*;
use crate::services::blog_post::BlogPost;
use crate::router::Route;
use chrono::{DateTime, Utc, TimeZone};

fn format_timestamp(ts: u64) -> String {
    // Convert u64 -> i64 safely (timestamps until year ~292 billion are fine)
    let dt: DateTime<Utc> = Utc.timestamp_opt(ts as i64, 0)
        .single()                   // or .unwrap() if you're sure it's valid
        .unwrap_or_else(|| Utc::now()); // fallback if timestamp is invalid

    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

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
            <h4> {"Last Updated: "} {format_timestamp((&props).post.created_at)} </h4>

            <p>{ &post.excerpt }</p>

        </div>
    }
}