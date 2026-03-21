use nostr_sdk::Event;
use crate::markdown_utils::*;
use crate::nostr::post_tags::PostTags;
use crate::nostr::client_error::ClientError;


#[derive(Clone, PartialEq)]
pub struct BlogPost {
    pub slug: String,
    pub created_at: u64,
    pub title: String,
    pub excerpt: String,
    pub content: String,
}

impl BlogPost {
    fn first_n_chars(s: &str, n: usize) -> &str {
        s.char_indices().nth(n).map(|(i, _)| &s[..i]).unwrap_or(s)
    }

    fn extract_excerpt(content: &String) -> String {
        static MAX_POST_PREVIEW_SIZE : usize = 512;
        let post_slice = BlogPost::first_n_chars(&content, MAX_POST_PREVIEW_SIZE);
        remove_headers(post_slice)
    }

    pub fn from_event(e: &Event) -> Result<Self, ClientError> {

        let post_tags = PostTags::new(e);
        let post_slug = post_tags.d().ok_or(ClientError::MissingTagError(format!("Error in parsing d tag")))?;
        let post_title = post_tags.title().ok_or(ClientError::MissingTagError(format!("Error in parsing title tag")))?;
        // TODO if hte event contains a summary, use that.
        let excerpt = BlogPost::extract_excerpt(&e.content);

        Ok(Self {
            slug: post_slug.to_string(),
            created_at: e.created_at.as_secs(),
            title: post_title.to_string(),
            excerpt: excerpt,
            content: e.content.clone(),
        })
    }
}