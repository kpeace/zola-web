use nostr_sdk::PublicKey as NostrPublicKey;
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Debug)]
pub enum PublicKeyError {
    ParsingError,
}

#[derive(Clone, Debug)]
pub struct PublicKey(NostrPublicKey);

// converting publicKey <-> NostrPublicKey
impl From<nostr_sdk::PublicKey> for PublicKey {
    fn from(pk: nostr_sdk::PublicKey) -> Self {
        Self(pk)
    }
}

impl From<PublicKey> for nostr_sdk::PublicKey {
    fn from(pk: PublicKey) -> Self {
        pk.0
    }
}

impl Default for PublicKey {
    fn default() -> Self {
        Self(
            NostrPublicKey::from_byte_array([0u8; 32])
        )
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for PublicKey {}

impl PartialOrd for PublicKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for PublicKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Hash for PublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        NostrPublicKey::from_str(&s)
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}

impl PublicKey {
    pub fn new(pub_key: NostrPublicKey) -> Self {
        Self(pub_key)
    }

    pub fn as_nostr(&self) -> &NostrPublicKey {
        &self.0
    }

    pub fn from_str(s: &str) -> Result<Self, PublicKeyError> {
        match NostrPublicKey::parse(s) {
            Ok(pub_key) => Ok(Self(pub_key)),
            Err(_) => Err(PublicKeyError::ParsingError)
        }
    }

    pub fn to_hex(&self) -> String { self.0.to_hex() }
}