use nostr_sdk::Event;
use crate::nostr::client_error::ClientError;
use serde::{Deserialize};
use crate::services::public_key::PublicKey;

#[derive(Clone, Debug, Default)]
pub struct LoginInfo {
    pub pub_key: PublicKey,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub about: Option<String>,
    pub picture: Option<String>,
}

#[derive(Deserialize)]
struct Profile {

    name: Option<String>,
    display_name: Option<String>,
    about: Option<String>,
    picture: Option<String>,

    #[allow(dead_code)]
    banner: Option<String>,

    #[allow(dead_code)]
    website: Option<String>,

    #[allow(dead_code)]
    nip05: Option<String>,

    #[allow(dead_code)]
    lud16: Option<String>,
}

impl LoginInfo {
    pub fn from_event(event: &Event) -> Result<Self, ClientError> {
        let profile: Profile = serde_json::from_str(&event.content)
            .map_err(|e| ClientError::JsonParseError(e))?;

        Ok(Self {
            pub_key: PublicKey::new(event.pubkey),
            name: profile.name,
            display_name: profile.display_name,
            about: profile.about,
            picture: profile.picture,
        })
    }

    pub fn short_pub_key(&self,) -> String {
        let pub_key_str = self.pub_key.to_hex();

        if pub_key_str.len() <= 8 {
            pub_key_str
        } else {
            std::format!("{}...{}", &pub_key_str[..4], &pub_key_str[pub_key_str.len() - 4..])
        }
    }

}