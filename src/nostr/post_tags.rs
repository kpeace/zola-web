use nostr_sdk::{Event, TagStandard};
pub struct PostTags<'a> {
    event: &'a Event,
}

impl<'a> PostTags<'a> {
    pub fn new(event: &'a Event) -> Self {
        Self { event }
    }

    pub fn d(&self) -> Option<&str> {
        self.event.tags.iter().find_map(|tag| {
            match tag.as_standardized() {
                Some(TagStandard::Identifier(v)) => Some(v.as_str()),
                _ => None,
            }
        })
    }

    pub fn title(&self) -> Option<&str> {
        self.event.tags.iter().find_map(|tag| {
            match tag.as_standardized() {
                Some(TagStandard::Title(v)) => Some(v.as_str()),
                _ => None,
            }
        })
    }
}