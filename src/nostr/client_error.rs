use nostr_sdk::prelude::*;

#[derive(Debug)]
pub enum ClientError {
    KeyError(nostr::key::Error),
    RelayError(nostr_sdk::client::Error),
    EventLoadingError(nostr_sdk::client::Error),
    MissingTagError(String),
}


impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ClientError::KeyError(ref e) => write!(f, "Key error: {}", e),
            ClientError::RelayError(ref e) => write!(f, "Relay error: {}", e),
            ClientError::EventLoadingError(ref e) => write!(f, "Event Loading error: {}", e),
            ClientError::MissingTagError(ref s) => write!(f, "Missing tag errro: {}", s),
        }
    }
}