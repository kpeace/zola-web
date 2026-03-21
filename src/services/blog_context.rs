use std::rc::Rc;
use crate::services::blog_post::BlogPost;
use crate::nostr::client::NostrClient;
use yew::prelude::*;



#[derive(Clone, PartialEq)]
pub struct BlogState {
    pub posts: Rc<Vec<BlogPost>>,
    pub oldest_timestamp: Option<u64>,
}
#[derive(Clone, PartialEq)]
pub struct BlogContext {
    pub client: Rc<NostrClient>,
    pub state: UseStateHandle<BlogState>,
}
