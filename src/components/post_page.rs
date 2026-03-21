use yew::prelude::*;
use pulldown_cmark::{Options, Parser};
use crate::services::blog_context::BlogContext;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub slug: String,
}

#[function_component(PostPage)]
pub fn post_page(props: &Props) -> Html {
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");

    let posts = &(*ctx.state).posts;
    match (*posts).iter().find(|p| p.slug == props.slug) {
        Some(post) => {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_FOOTNOTES);
            options.insert(Options::ENABLE_OLD_FOOTNOTES);

            let parser = Parser::new_ext(&post.content, options);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            let parsed = Html::from_html_unchecked(AttrValue::from(html_output));

            html! {
            <div>
                <h1>{ &post.title }</h1>
                <div>{ &parsed }</div>
            </div>
            }
        },
        None => html! {
            <div>{ "Post not found" }</div>
        }
}
}