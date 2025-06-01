use yew::prelude::*;
use nostr_sdk::prelude::*;
use pulldown_cmark::{Parser, Options};
use crate::markdown_utils::*;

static MAX_POST_PREVIEW_SIZE : usize = 512;

#[derive(Properties, PartialEq)]
pub struct PoststProps {
    //post: Post,
    //pub title: String,
    //pub body : String,
    pub event: nostr_sdk::Event,
}

fn first_n_chars(s: &str, n: usize) -> &str {
    s.char_indices().nth(n).map(|(i, _)| &s[..i]).unwrap_or(s)
}

// Creates a snippet of the post for preview
#[function_component(PostsList)]
pub fn posts_list_to_html(post : &PoststProps) -> Html {
    // lets get the title
    let title = match post.event.get_tag_content(TagKind::Title) {
        Some(title) => title,
        None => "missing title",
    };

    // lets get the word count
    let word_count = post.event.content.split_whitespace().count();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_OLD_FOOTNOTES);

    // pulldown_cmark doesn't support pandoc inline footnote. Convert them if exist
    let strict_markdown_text = preprocess_inline_footnotes(&post.event.content);

    // we don't want to show tho whole post, just a preview
    let post_slice = first_n_chars(&strict_markdown_text, MAX_POST_PREVIEW_SIZE);

    // headers make preview look ugly
    let clean_post = remove_headers(post_slice);

    let parser = Parser::new_ext(&clean_post, options);

    // Write to String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    let parsed = Html::from_html_unchecked(AttrValue::from(html_output));
    //let parsed = Html::(AttrValue::from(strict_markdown_text));
    html! {
        <>
        <h2>{title} {"        ("} {word_count} {")"}</h2>
        // todo cut the post to X chars
        <div>{parsed}</div>
        //<div>{ "some text" }</div>
        </>
    }
}


