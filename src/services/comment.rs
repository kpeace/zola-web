use gloo_console::log;
use nostr_sdk::{Event, Kind, TagKind, SingleLetterTag, Alphabet, Tag, EventBuilder};
use crate::nostr::client_error::ClientError;
use std::collections::HashMap;
use crate::nostr::tag_types::*;


#[derive(Clone, PartialEq, Default, )]
pub struct Comment {
    pub id: String,
    pub root_slug: String,
    pub root_author: String,
    pub parent_id: String,
    pub parent_author: String,
    pub content: String,
    pub comment_author: String,
    pub created_at: u64,
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Comment by {}: {}",
            self.comment_author,
            self.content
        )
    }
}

impl Comment {
    pub fn to_event_builder(&self) -> Result<EventBuilder, ClientError> {

        let root_ref = format!(
            "30023:{}:{}",
            self.root_author,
            self.root_slug
        );

        fn create_tag(tag_name: &str, value: &str) -> Result<Tag,ClientError> {
            Tag::parse([tag_name, value])
                .map_err(|e| ClientError::TagFormatError(format!("{}",tag_name), e.to_string()))
        }

        let tags = vec![
            create_tag("A", &root_ref)?,
            create_tag("K", "30023")?,
            create_tag("P", &self.root_author)?,
            create_tag("e", &self.parent_id)?,
            create_tag("p", &self.parent_author)?,
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
            Kind::Custom(1111) => {
                // NIP-22 comment
                Comment::from_event_1111(e)
            },
            _ => {
                Err(ClientError::UnsuportedComment(format!("Kind {} replies are not supported as of now. Only kind 1111 (NIP-22", e.kind)))
            },

        }

    }

    fn from_event_1111(e: &Event) -> Result<Self, ClientError> {
        let id = e.id.to_hex();
        let content = e.content.clone();
        let created_at = e.created_at;
        let comment_author = e.pubkey.to_hex();
        let mut root_slug: String  = String::new();
        let mut root_author: String = String::new();
        let mut parent_id: String = String::new();
        let mut parent_author: String = String::new();

        for tag in e.tags.clone() {
            let values = tag.as_slice();

             match values {
                [kind, value, ..] if kind == "A" => {
                    root_slug = parse_a_tag(value)?;
                },
                [kind, value, ..] if kind == "P" => {
                    root_author = parse_p_tag(value)?;
                },
                [kind, value, ..] if kind == "a" => {
                    parent_id = parse_a_tag(value)?;
                },
                [kind, value, ..] if kind == "p" => {
                    parent_author = parse_p_tag(value)?;
                },
                &[] | &[_] | &[_, _, ..] => todo!(),
            }
        }

        Ok(Comment {
                id : id,
                root_slug: root_slug,
                root_author: root_author,
                parent_id: parent_id,
                parent_author: parent_author,
                content: content,
                comment_author: comment_author,
                created_at: created_at.as_secs(),
            }
        )

    }
}

fn parse_a_tag(value: &str) -> Result<String, ClientError> {
    let value_vec: Vec<&str> = value.split(':').collect();

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

// TODO use real implementation (see below)
pub fn build_tree(comments: Vec<Comment>) -> Vec<CommentNode> {
    return vec!(
        CommentNode {
            comment: Comment {
                id: format!("fake_id"),
                root_slug: format!("fake_slug"),
                root_author: format!("fake_root_author"),
                parent_id: format!("fake_parent_id"),
                parent_author: format!("fake_parent_author"),
                content: format!("fake content. very smart content"),
                comment_author: format!("fake_comment_author"),
                created_at: 112233445566,
            },
            children: Vec::new()
        }
    );
}

// pub fn build_tree(comments: Vec<Comment>) -> Vec<CommentNode> {
//     let mut map: HashMap<String, Vec<Comment>> = HashMap::new();

//     for c in comments {
//         map.entry(c.parent_id.clone())
//             .or_default()
//             .push(c);
//     }

//     fn build(parent: String, map: &HashMap<String, Vec<Comment>>) -> Vec<CommentNode> {
//         map.get(&parent)
//             .unwrap_or(&vec![])
//             .iter()
//             .map(|c| CommentNode {
//                 comment: c.clone(),
//                 children: build(c.id.clone(), map),
//             })
//             .collect()
//     }

//     build(None, &map)
// }

#[derive(Clone, PartialEq)]
pub struct CommentNode {
    pub comment: Comment,
    pub children: Vec<CommentNode>,
}