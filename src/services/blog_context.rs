use std::rc::Rc;
use std::sync::{Arc, RwLock};
use crate::services::blog_post::BlogPost;
use crate::nostr::client::NostrClient;
use std::collections::HashMap;
use gloo_console::log;
use yew::prelude::*;

pub const WINDOW_SIZE: usize = 5;

#[derive(Clone, PartialEq)]
pub struct PostWindow {
    pub posts: Rc<Vec<BlogPost>>,
    pub oldest_timestamp: Option<u64>,
}

pub type WindowCache = Arc<RwLock<HashMap<i32, PostWindow>>>;

pub fn new_window_cache() -> WindowCache {
    Arc::new(RwLock::new(HashMap::new()))
}

#[derive(Clone, PartialEq)]
pub struct BlogState {
    pub current_window: i32,
}

#[derive(Clone)]
pub struct BlogContext {
    pub client: Rc<NostrClient>,
    pub state: UseStateHandle<BlogState>,
    pub windows: WindowCache,
}

impl PartialEq for BlogContext{
    fn eq(&self, other: &Self) -> bool {
        self.client == other.client && self.state == other.state
    }
}

impl BlogContext {
    // Curently we only support loading windows not unloading windows
    pub async fn load_window(&self, window_id: i32, ){

        let oldest_timestamp = {
            let windows_guard = self.windows.read().unwrap();
            if windows_guard.contains_key(&window_id) {
                return ;
            }

            match windows_guard.keys().max().copied() {
                Some(key) => {
                    match windows_guard.get(&key) {
                        Some(post_window) => post_window.oldest_timestamp,
                        None => None,
                    }
                },
                None => None,
            }

        };

        let posts: Vec<BlogPost> = match self.client.load_window(None, oldest_timestamp).await {
            Ok(events) => {
                events
                    .into_iter()
                    .filter_map(|e| {
                        match BlogPost::from_event(&e) {
                            Ok(other_post) => Some(other_post),
                            Err(err) => {
                                log!(format!("Skipping invalid event: {:?}", err));
                                None
                            }
                        }
                    })
                    .collect()
            }
            Err(err) => {
                log!(format!("Error loading new events: {}", err));
                Vec::new()
            }
        };

        self.insert_window(window_id, posts);
    }

    pub async fn get_post_by_slug(&self, slug: &str) -> Option<BlogPost> {
        match self.client.get_post_by_identifier(slug).await {
            Ok(events) => {
                match events.first() {
                    Some(e) => {
                        match BlogPost::from_event(&e) {
                            Ok(other_post) => {Some(other_post)},
                            Err(err) => {
                                log!(format!("Error parsing event '{}': {}", slug, err));
                                None
                            },
                        }
                    },
                    _ => None
                }
            },
            Err(err) => {
                log!(format!("Error loading post '{}': {}", slug, err));
                None
            }
        }
    }

    pub fn insert_window(&self, window_id: i32, posts: Vec<BlogPost>) {
        let mut sorted_posts = posts.clone();
        sorted_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let oldest = match sorted_posts.last() {
            Some(p) => Some(p.created_at),
            None => None,
        };

        let mut guard = self.windows.write().unwrap();
        guard.insert(window_id, PostWindow {
            posts: Rc::new(sorted_posts),
            oldest_timestamp: oldest,
        });
    }

    pub fn get_oldest(&self, window_id: i32,) -> Option<u64>{
        let guard = self.windows.read().unwrap();
        match guard.get(&window_id) {
            Some(window) => window.oldest_timestamp,
            None => None,
        }
    }

    pub fn get_newest(&self,) -> Option<u64>{
        let guard = self.windows.read().unwrap();
        match guard.get(&0) {
            Some(window) => window.posts.first().and_then(|post| Some(post.created_at)),
            None => None,
        }
    }

    pub fn add_post(&self, new_post: BlogPost) {
        let mut windows_guard = self.windows.write().unwrap();
        let mut k = 0;

        while let Some(window) = windows_guard.get_mut(&k) {
            if let Some(oldest) = window.oldest_timestamp {
                log!(format!("checking window {}. Oldest post {}. Post create_at: {}", k, oldest, new_post.created_at));
                if new_post.created_at >= oldest {
                    // This clones the Vec only when necessary (when Rc has multiple references)
                    let posts = Rc::make_mut(&mut window.posts);

                    let pos = posts.binary_search_by(|existing| {
                        new_post.created_at.cmp(&existing.created_at)
                    }).unwrap_or_else(|e| e);

                    posts.insert(pos, new_post);
                    log!(format!("new post added to window {}", k));

                    return;
                }
            }
            k += 1;
        }
    }
}
