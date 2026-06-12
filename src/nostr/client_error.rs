use nostr_sdk::prelude::*;

#[derive(Debug)]
pub enum ClientError {
    KeyError(nostr::key::Error),
    RelayError(nostr_sdk::client::Error),
    EventLoadingError(nostr_sdk::client::Error),
    MissingTagError(String),
    TagFormatError(String, String),
    UnsuportedComment(String),
    EventBuildError(nostr_sdk::event::builder::Error),
}


impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ClientError::KeyError(ref e) => write!(f, "Key error: {}", e),
            ClientError::RelayError(ref e) => write!(f, "Relay error: {}", e),
            ClientError::EventLoadingError(ref e) => write!(f, "Event Loading error: {}", e),
            ClientError::MissingTagError(ref s) => write!(f, "Missing tag errro: {}", s),
            ClientError::TagFormatError(ref t, ref s) => write!(f, "Error in tag {}: {}", t, s),
            ClientError::UnsuportedComment(ref s) => write!(f, "Unsuported comment {}", s),
            ClientError::EventBuildError(ref e) => write!(f, "Error building event: {}", e),
        }
    }
}