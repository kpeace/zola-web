use web_sys::window;
use nostr_sdk::prelude::*;
use std::time::Duration;
use yew::prelude::*;
use gloo_console::log;


use zola_client::post_preview::*;

const PUBLIC_KEY_STR : &str = "1a15a86100f0d0472b9e679a2745be737c0a804cc773573dfa4b2d1990427d66";



async fn fetch_posts() -> Result<Vec<nostr::Event>, String> {
    console_error_panic_hook::set_once();

    let document = window()
            .and_then(|win| win.document())
            .expect("Could not access the document");
        let body = document.body().expect("Could not access document.body");

        let public_key = match PublicKey::parse(PUBLIC_KEY_STR) {
            Ok(key) => key,
            Err(error) => {
                let text_node = document.create_text_node(format!("{error}").as_str());

                body.append_child(text_node.as_ref())
                    .expect("Failed to append text");
                return Err(format!("Error parsing public key {}", PUBLIC_KEY_STR));
            }
            
        };

        let sign_key = Keys::generate();

        let opts = Options::new()
            .skip_disconnected_relays(true)
            .connection_timeout(Some(Duration::from_secs(10)))
            .send_timeout(Some(Duration::from_secs(10)));

        let client = Client::with_opts(&sign_key, opts);

        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.add_relay("wss://relay.nostr.band").await.unwrap();

        client.connect().await;

        let filter = Filter::new()
            .author(public_key)
            //.since(Timestamp::from_secs(1729261000))
            .since(Timestamp::from_secs(0))
            .kind(Kind::LongFormTextNote)
            .limit(5)
            ;

        let timeout = Some(Duration::from_secs(10));
        let fetch_result = client
        .get_events_of(vec![filter], EventSource::both(timeout))
        .await;

        client.disconnect().await.unwrap();

        match fetch_result {
            Err(error) => {
                let text_node = document.create_text_node(format!("{error}").as_str());

                body.append_child(text_node.as_ref())
                    .expect("Failed to append text");
                return Err(format!("{error}"));
            },
            Ok(events) => {
                log!(format!("Fetched {} events from relays", events.len()));
                return Ok(events);
            }
        }
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(String),
}

type PostsFetchState = FetchState<Vec<nostr::Event>>;

enum Msg {
    SetPostsListFetchState(PostsFetchState),
    GetPosts,
}

struct App {
    posts: PostsFetchState,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            posts: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetPostsListFetchState(posts_fetch_state) => {
                self.posts = posts_fetch_state;
                true
            },
            Msg::GetPosts => {
                ctx.link().send_future(async {
                    match fetch_posts().await {
                        Ok(posts) => Msg::SetPostsListFetchState(PostsFetchState::Success(posts)),
                        Err(error) => Msg::SetPostsListFetchState(FetchState::Failed(error)),
                    }
                });

                ctx.link()
                    .send_message(Msg::SetPostsListFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.posts {
            PostsFetchState::Failed(error) => html! {
                <>
                    {format!("Failed to get posts: {}", error)}
                </>
            },
            PostsFetchState::Fetching => html! {
                <>
                    {"Fetching in progess"}
                </>
            },
            PostsFetchState::NotFetching => {
                ctx.link().send_message(Msg::GetPosts);
                html! {
                <>
                    {"Stuck in pre loading phase..."}
                </>
                }
            },
            PostsFetchState::Success(events) => html! {
                <>
                    //{format!("Got {} events", posts.len())}
                    {
                        for events.iter().map(|event| {
                            html! {
                                <PostsList event={event.clone()} />
                            }

                        })
                    }
                </>
            },
        }
    }
}

fn main() {
    // Init logger
    tracing_wasm::set_as_global_default();

    // Start WASM app
    yew::Renderer::<App>::new().render();
}
