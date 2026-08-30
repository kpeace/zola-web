use nostr_sdk::prelude::*;
use nostr_browser_signer::BrowserSigner;
use std::rc::Rc;
use std::time::Duration;
use crate::services::blog_post::BlogPost;
use crate::nostr::client_error::ClientError;
use crate::services::blog_context::WINDOW_SIZE;
use crate::services::comment::Comment;
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
    pub fn new(public_key: &str) -> Result<Self, ClientError>  {
        let id = PublicKey::parse(public_key).map_err(|e| ClientError::KeyError(e))?;
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
        let relay = "ws://127.0.0.1:7000";
        self.client
            .add_relay("ws://127.0.0.1:7000")
            .await
            .map_err(|e| ClientError::RelayError(e))?;

        self.client.connect().await;
        log!(format!("Conneces to relay: {}", relay));

        Ok(())
    }

    pub async fn load_posts(&self, limit: usize, ) -> Result<Events, ClientError> {
        let filter = Filter::new()
            .author(self.blog_id)
            .kind(Kind::LongFormTextNote)
            .limit(limit);

        let events = self.client.fetch_events(filter, Duration::from_secs(5)).await.map_err(|e| ClientError::EventLoadingError(e))?;

        log!(format!("Loaded {} events", events.len()));

        Ok(events)
    }

    pub async fn load_window(&self, newer_then: Option<u64>, older_then: Option<u64>) -> Result<Events, ClientError> {
        let limit = WINDOW_SIZE as usize;

        let mut filter = Filter::new()
            .author(self.blog_id)
            .kind(Kind::LongFormTextNote)
            .limit(limit);

        match newer_then {
            Some(since) => {filter = filter.since(Timestamp::from_secs(since));},
            None => {},
        }

        match older_then {
            Some(until) => {filter = filter.until(Timestamp::from_secs(until));},
            None => {},
        }

        let events = self.client.fetch_events(filter, Duration::from_secs(5)).await.map_err(|e| ClientError::EventLoadingError(e))?;

        log!(format!("Loaded window with {} events", events.len()));

        Ok(events)
    }

    pub async fn get_post_by_identifier(&self, slug: &str) -> Result<Events, ClientError> {
        let filter = Filter::new()
            .author(self.blog_id)
            .kind(Kind::LongFormTextNote)
            .identifier(slug)
            .limit(1);

        let events = self.client.fetch_events(filter, Duration::from_secs(5)).await.map_err(|e| ClientError::EventLoadingError(e))?;

        Ok(events)
    }

    pub async fn subscribe_posts(&self, since: u64, tx: futures::channel::mpsc::UnboundedSender<BlogPost>) {
        let post_already_read_ts = Timestamp::from_secs(since);
        let filter = Filter::new()
            .author(self.blog_id)
            .kind(Kind::LongFormTextNote)
            .since(post_already_read_ts);

        // Now subscribe for live updates
        let _ = self.client.subscribe(filter, None).await;

        let mut notifications = self.client.notifications();

        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event { event, .. } =  notification {
                if event.created_at <= post_already_read_ts {
                    continue;
                }

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

    // event_id is the regular event id. post_id is the d_tag id used for addresable events
    pub async  fn get_replies(&self, post_id: &str, author_pubkey: &str,) -> Result<Events, ClientError> {
        let target = format!("30023:{}:{}", author_pubkey, post_id);

        let filter =  Filter::new()
                .kinds(vec![Kind::TextNote, Kind::Custom(1111)])
                .custom_tag(SingleLetterTag::uppercase(Alphabet::A), &target);

        let events = self.client.fetch_events(filter.clone(), Duration::from_secs(5)).await.map_err(|e| ClientError::EventLoadingError(e))?;
        log!(format!("Got {} comments to post {}, with filter {}", events.len(), post_id, filter.try_as_json().unwrap()));

        Ok(events)
    }

    pub async fn login(&self, )-> Result<nostr_sdk::PublicKey, ClientError> {
        let signer = BrowserSigner::new()
            .map_err(|e| ClientError::SignerError(e))?;
        let pub_key = signer.get_public_key()
            .await
            .map_err(|e| ClientError::SignerKeyError(e))?;
        self.client.set_signer(signer).await;
        Ok(pub_key)
    }

    pub async fn publish_reply(&self, comment: Comment) -> Result<(),ClientError> {
        // TODO sign with nostr signer plugin
        let secret_key = SecretKey::parse("e5af1806656d1234ecc6a64d878f198c445a4e935c00ee3e201b8ff8c0e99073")
            .map_err(|e| ClientError::KeyError(e))?;
        let keys = Keys::new(secret_key);
        let event = comment.to_event_builder()?
            .sign_with_keys(&keys).map_err(|e| ClientError::EventBuildError(e))?;
        let send_result = self.client.send_event(&event)
            .await
            .map_err(|e| ClientError::RelayError(e))?;

        log!(format!("{:#?}", send_result));

        // send result contains sending status foreach relay.
        // We will declare an error only if reply was not sent to any relay
        if send_result.success.is_empty() {
            let json_error_map = serde_json::to_string(&send_result.failed)
                .map_err(|e|ClientError::SendCommentError(
                    format!("Sending to all relays failed. In addition failed toconvert error map to string: {}", e))
                )?;
            return Err(ClientError::SendCommentError(format!("Failed to send comment to all relays! Error map: {}", json_error_map)));
        }
        Ok(())
    }
}