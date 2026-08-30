use std::collections::HashMap;

use nostr_sdk::{Event, Kind, Tag, EventBuilder};
use crate::nostr::client_error::ClientError;
use gloo_console::log;
use serde::{Deserialize, Serialize};


#[derive(Clone, PartialEq, Default, Serialize, Deserialize, )]
pub struct Comment {
    pub id: String,
    pub root_slug: String,
    pub root_author_id: String,
    pub parent_id: String,
    pub parent_author_id: String,
    pub content: String,
    pub comment_author: String,
    pub created_at: u64,
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(
            f,
            "Comment by {}: {}",
            self.comment_author,
            self.content
        )
    }
}

impl Comment {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_event_builder(&self) -> Result<EventBuilder, ClientError> {

        let root_ref = format!(
            "30023:{}:{}",
            self.root_author_id,
            self.root_slug
        );

        fn create_tag(tag_name: &str, value: &str) -> Result<Tag,ClientError> {
            Tag::parse([tag_name, value])
                .map_err(|e| ClientError::TagFormatError(format!("{}",tag_name), e.to_string()))
        }

        let tags = vec![
            create_tag("A", &root_ref)?,
            create_tag("K", "30023")?,
            create_tag("P", &self.root_author_id)?,
            create_tag("e", &self.parent_id)?,
            create_tag("p", &self.parent_author_id)?,
            create_tag("k", "1111")?,
        ];

        Ok(
            EventBuilder::new(
                Kind::Custom(1111),
                &self.content,
            )
            .tags(tags)
        )
    }

    pub fn from_event(e: &Event) -> Result<Self, ClientError> {
        match e.kind {
            Kind::TextNote => {
                // kind 1 reply expecting an "a" tag with: 30023:<AUTHOR>:<D_TAG>;root
                Err(ClientError::UnsuportedComment(format!("Kind 1 replies are not supported as of now. Only kind 1111 (NIP-22")))

            },
            Kind::Comment => {
                // NIP-22 comment
                Comment::from_event_1111(e)
            },
            _ => {
                Err(ClientError::UnsuportedComment(format!("Kind {} replies are not supported as of now. Only kind 1111 (NIP-22)", e.kind)))
            },

        }

    }

    fn from_event_1111(e: &Event) -> Result<Self, ClientError> {
        let id = e.id.to_hex();
        let content = e.content.clone();
        let created_at = e.created_at;
        let comment_author = e.pubkey.to_hex();
        let mut root_slug: String  = String::new();
        let mut root_author_id: String = String::new();
        let mut parent_id: String = String::new();
        let mut parent_author_id: String = String::new();

        for tag in e.tags.clone() {
            let values = tag.as_slice();
            //DEBUG
            log!(format!("parsing event 1111 with tag {:?}", values));

             match values {
                [kind, value, ..] if kind == "A" => {
                    root_slug = parse_a_tag(value)?;
                },
                [kind, value, ..] if kind == "P" => {
                    root_author_id = parse_p_tag(value)?;
                },
                [kind, value, ..] if kind == "a" => {
                    parent_id = parse_a_tag(value)?;
                },
                [kind, value, ..] if kind == "e" => {
                    parent_id = parse_e_tag(value)?;
                },
                [kind, value, ..] if kind == "p" => {
                    parent_author_id = parse_p_tag(value)?;
                },
                [kind, value, ..] if kind == "K" => {
                    ()
                },
                [kind, value, ..] if kind == "k" => {
                    ()
                },
                [kind, value, ..] => {
                    return Err(ClientError::UnsuportedComment(
                        format!(
                            "Comment id {} has unsupported tag kind='{}' value='{}'",
                            id, kind, value
                        )
                    ));
                }

                [] => {
                    return Err(ClientError::UnsuportedComment(
                        format!("Comment id {} has an empty tag", id)
                    ));
                }

                [_] => {
                    return Err(ClientError::UnsuportedComment(
                        format!("Comment id {} has a malformed tag with only one element", id)
                    ));
                }
            }
        }

        Ok(Comment {
                id : id,
                root_slug: root_slug,
                root_author_id: root_author_id,
                parent_id: parent_id,
                parent_author_id: parent_author_id,
                content: content,
                comment_author: comment_author,
                created_at: created_at.as_secs(),
            }
        )

    }
}

fn parse_a_tag(value: &str) -> Result<String, ClientError> {
    let value_vec: Vec<&str> = value.splitn(3, ':').collect();

    if value_vec.len() < 3 {
        return Err(ClientError::TagFormatError(String::from("A"), String::from("Value Doens't conatin 3 parts")));
    }

    if value_vec[0] != "30023" {
        return Err(ClientError::UnsuportedComment(format!("Root is not a Long Format Event, its: {}", value_vec[0])));
    }

    Ok(String::from(value_vec[2]))
}

fn parse_p_tag(value: &str) -> Result<String, ClientError> {
    if value.is_empty() {
        Err(ClientError::TagFormatError(format!("P"), format!("value is empty")))
    } else {
        Ok(value.to_string())
    }
}

fn parse_e_tag(value: &str) -> Result<String, ClientError> {
     if value.is_empty() {
        return Err(ClientError::TagFormatError(format!("E"), format!("value is empty")))
    }

    Ok(value.to_string())
}

pub fn build_tree(comments: Vec<Comment>) -> Vec<CommentNode> {
    let mut roots = Vec::new();
    let mut comments_by_parent: HashMap<String, Vec<Comment>> = HashMap::new();

    for comment in comments {
        if comment.parent_id == comment.root_slug {
            roots.push(comment);
        } else {
            comments_by_parent
                .entry(comment.parent_id.clone())
                .or_default()
                .push(comment);
        }
    }

    fn build_subtree(
        comment: Comment,
        comments_by_parent: &mut HashMap<String, Vec<Comment>>,
    ) -> CommentNode {
        let children = comments_by_parent
            .remove(&comment.id)
            .unwrap_or_default()
            .into_iter()
            .map(|child| build_subtree(child, comments_by_parent))
            .collect();

        CommentNode { comment, children }
    }

    roots
        .into_iter()
        .map(|root| build_subtree(root, &mut comments_by_parent))
        .collect()
}

#[derive(Clone, PartialEq)]
pub struct CommentNode {
    pub comment: Comment,
    pub children: Vec<CommentNode>,
}

impl CommentNode {

}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {

    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_single_elmnt_tree() {
        let comment = Comment::from_json(r#"{"id":"423fcf5292d51bea1d81a04399a6cb6bd072e479b0ee7ee6b3ac438339fd463c","root_slug":"The_Economic_Necessity_of_Population_Growth_2026-04-11T07:26:20.385616134+00:00","root_author_id":"8e43961e8e488784a4046735a5b5863b39a2b51273a537aae277536fab94700c","parent_id":"The_Economic_Necessity_of_Population_Growth_2026-04-11T07:26:20.385616134+00:00","parent_author_id":"8e43961e8e488784a4046735a5b5863b39a2b51273a537aae277536fab94700c","content":"Fixing parent author id (\"p\" tag) issue","comment_author":"72c49b5ef90b0bcf86d614d34f0ce4626332affde4269eede1aaf7882d9502dc","created_at":1782061798}"#)
            .unwrap();
        let comment_vec = vec![comment];
        let tree = build_tree(comment_vec);

        assert_eq!(tree.len(), 1);
    }
}