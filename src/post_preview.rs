use yew::prelude::*;
use nostr_sdk::prelude::*;
use pulldown_cmark::{Parser, Options};
use crate::markdown_utils::*;

#[derive(Properties, PartialEq)]
pub struct PoststProps {
    //post: Post,
    //pub title: String,
    //pub body : String,
    pub event: nostr_sdk::Event,
}

#[function_component(PostsList)]
pub fn posts_list_to_html(post : &PoststProps) -> Html {
    let title = match post.event.get_tag_content(TagKind::Title) {
        Some(title) => title,
        None => "missing title",
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_OLD_FOOTNOTES);

    let strict_markdown_text = preprocess_inline_footnotes(&post.event.content);
    let parser = Parser::new_ext(&strict_markdown_text, options);

    // Write to String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    let parsed = Html::from_html_unchecked(AttrValue::from(html_output));
    //let parsed = Html::(AttrValue::from(strict_markdown_text));
    html! {
        <>
        <h2>{title}</h2>
        // todo cut the post to X chars
        <div>{parsed}</div>
        //<div>{ "some text" }</div>
        </>
    }
}


