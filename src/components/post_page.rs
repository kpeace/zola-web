use yew::prelude::*;
use pulldown_cmark::{Options, Parser};
use crate::services::blog_context::BlogContext;
use crate::services::blog_post::BlogPost;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub slug: String,
}

#[function_component(PostPage)]
pub fn post_page(props: &Props) -> Html {
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");

    let post = use_state(|| None::<BlogPost>);
    let loading = use_state(|| true);

    let slug = props.slug.clone();
    let ctx_clone = ctx.clone();
    let post_clone = post.clone();
    let loading_clone = loading.clone();

    use_effect_with(slug.clone(), move |_| {

        let cached = ctx_clone.windows
            .read()
            .unwrap()
            .values()
            .flat_map(|w| w.posts.iter())
            .find(|p| p.slug == slug)
            .cloned();

        if let Some(p) = cached {
            post_clone.set(Some(p));
            loading_clone.set(false);
        } else {
            let slug_cloned: String = slug.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match ctx.get_post_by_slug(&slug_cloned).await {
                    Some(p) => post_clone.set(Some(p)),
                    None => (),
                }

                loading_clone.set(false);
            });
            ()
        }
    });

    if *loading {
        return html! { <div>{ "Loading..." }</div> };
    }

    match &*post {
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
        }
        None => html! {
            <div>{ "Post not found" }</div>
        }
    }
}
