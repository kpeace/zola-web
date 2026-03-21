use nostr_sdk::prelude::*;
use std::rc::Rc;
use std::time::Duration;
use crate::services::blog_post::BlogPost;
use crate::nostr::client_error::ClientError;
use gloo_console::log;

#[derive(Debug)]
pub struct NostrClient {
    blog_id: nostr_sdk::PublicKey,
    sign_keys: nostr_sdk::Keys,
    client: nostr_sdk::Client,
}

impl PartialEq for NostrClient {
    fn eq(&self, other: &Self) -> bool {
        self.blog_id == other.blog_id &&
        self.sign_keys == other.sign_keys
    }
}

pub type SharedClient = Rc<NostrClient>;

impl NostrClient {
    pub fn new() -> Result<Self, ClientError>  {
        // TODO need a way to make keys configurable
        const PUBLIC_KEY_STR : &str = "1a15a86100f0d0472b9e679a2745be737c0a804cc773573dfa4b2d1990427d66";
        let id = PublicKey::parse(PUBLIC_KEY_STR).map_err(|e| ClientError::KeyError(e))?;
        let keys = Keys::generate();

        let client = Client::builder()
            .signer(keys.clone())
            .build();

        Ok(Self {
            blog_id: id,
            sign_keys: keys,
            client: client,
        })
    }

    pub async fn connect(&self) -> Result<(), ClientError> {
        self.client
            .add_relay("ws://127.0.0.1:7000")
            .await
            .map_err(|e| ClientError::RelayError(e))?;

        self.client.connect().await;

        Ok(())
    }

    pub async fn load_posts(&self, limit: usize, ) -> Result<Events, ClientError> {
        let filter = Filter::new()
            .author(self.blog_id)
            .kind(Kind::LongFormTextNote)
            .limit(limit);

        let events = self.client.fetch_events(filter, Duration::from_secs(5)).await.map_err(|e| ClientError::EventLoadingError(e))?;

        Ok(events)
    }

    pub async fn subscribe_posts(&self, tx: futures::channel::mpsc::UnboundedSender<BlogPost>) {
        let filter = Filter::new()
            .author(self.blog_id)
            .kind(Kind::LongFormTextNote);

        // Fetch historical posts first
        if let Ok(events) = self.client.fetch_events(filter.clone(), Duration::from_secs(10)).await {
            for event in events {
                let post = match BlogPost::from_event(&event) {
                    Ok(post) => post,
                    Err(err) => {
                        log!(format!("Error: {}; Event: {}", err, event.as_json()));
                        continue;
                    }
                };
                // Debug
                log!("GOt event!");
                let _ = tx.unbounded_send(post);
            }
        }

        // Now subscribe for live updates
        let _ = self.client.subscribe(filter, None).await;

        let mut notifications = self.client.notifications();

        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event { event, .. } =  notification {
                let post = match BlogPost::from_event(&event) {
                    Ok(post) => post,
                    Err(err) => {
                        log!(format!("Error: {}; Event: {}", err, event.as_json()));
                        continue;
                    }
                };
                let _ = tx.unbounded_send(post);
            }
        }
    }
}