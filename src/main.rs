use web_sys::window;
use nostr_sdk::prelude::*;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const public_key_str : &str = "1a15a86100f0d0472b9e679a2745be737c0a804cc773573dfa4b2d1990427d66";

#[derive(Clone, PartialEq)]
struct Post {
    title: String,
    body : String,
}

#[derive(Properties, PartialEq)]
struct PostsListProps {
    posts: Vec<Post>,
}

#[function_component(PostsList)]
fn posts_list_to_html(PostsListProps {posts} : &PostsListProps) -> Html {
    posts.iter().map(|post| html! {
        <>
        <h2>{post.title.clone()}</h2>
        // todo cut the post to X chars
        <div>{post.body.clone()}</div>
        </>
    }).collect()
}

async fn fetch_posts() -> Result<Vec<nostr::Event>, String> {
    console_error_panic_hook::set_once();

    let document = window()
            .and_then(|win| win.document())
            .expect("Could not access the document");
        let body = document.body().expect("Could not access document.body");

        let public_key = match PublicKey::parse(public_key_str) {
            Ok(key) => key,
            Err(error) => {
                let text_node = document.create_text_node(format!("{error}").as_str());

                body.append_child(text_node.as_ref())
                    .expect("Failed to append text");
                return Err(format!("Error parsing public key {}", public_key_str));
            }
            
        };

        let sign_key = Keys::generate();

        let opts = Options::new()
            .skip_disconnected_relays(true)
            .connection_timeout(Some(Duration::from_secs(10)))
            .send_timeout(Some(Duration::from_secs(10)));

        let client = Client::with_opts(&sign_key, opts);

        client.add_relay("wss://relay.damus.io").await.unwrap();

        client.connect().await;

        let filter = Filter::new()
            .author(public_key)
            .since(Timestamp::from_secs(1729261000))
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
                //ctx.link().callback(|_:Msg|{Msg::GetPosts});
                ctx.link().send_message(Msg::GetPosts);
                html! {
                <>
                    {"Stuck in pre loading phase..."}
                </>
                }
            },
            PostsFetchState::Success(posts) => html! {
                <>
                    {format!("Got {} events", posts.len())}
                </>
            },
        }
    }
}

//#[function_component(App)]
//fn app() -> Html {
    /*spawn_local(async {
        console_error_panic_hook::set_once();

        

        let document = window()
            .and_then(|win| win.document())
            .expect("Could not access the document");
        let body = document.body().expect("Could not access document.body");

        let public_key = match 
        PublicKey::parse("1a15a86100f0d0472b9e679a2745be737c0a804cc773573dfa4b2d1990427d66") {
            Err(error) => {
                let text_node = document.create_text_node(format!("{error}").as_str());

                body.append_child(text_node.as_ref())
                    .expect("Failed to append text");
                return;
            },
            Ok(key) => key,
        };
posts_list
        let sign_key = Keys::generate();

        let opts = Options::new()
            .skip_disconnected_relays(true)
            .connection_timeout(Some(Duration::from_secs(10)))
            .send_timeout(Some(Duration::from_secs(10)));

        let client = Client::with_opts(&sign_key, opts);

        client.add_relay("wss://relay.damus.io").await.unwrap();

        client.connect().await;

        let filter = Filter::new()
            .author(public_key)
            .since(Timestamp::from_secs(1729261000))
            .kind(Kind::LongFormTextNote)
            .limit(5)
            ;

        let timeout = Some(Duration::from_secs(10));
        let fetch_result = client
        .get_events_of(vec![filter], EventSource::both(timeout))
        .await;

        client.disconnect().await.unwrap();

        let posts_list = match fetch_result {
            Err(error) => {
                let text_node = document.create_text_node(format!("{error}").as_str());

                body.append_child(text_node.as_ref())
                    .expect("Failed to append text");
                Vec::new()
            },
            Ok(events) => {
                let text_node = document.create_text_node(format!("Found {} events:", events.len()).as_str());
                    body.append_child(text_node.as_ref())
                        .expect("Failed to append text");
                for event in events {
                    
                    //let text_node = document.create_text_node(event.try_as_pretty_json().unwrap().as_str());
                    let text_node = document.create_text_node(event.get_tag_content(TagKind::Title).unwrap());
                    body.append_child(text_node.as_ref())
                        .expect("Failed to append text");
                }

                events.iter().map(|event| {
                    let title = event.get_tag_content(TagKind::Title).unwrap();
                    Post { title: title.to_string(), body : event.content}
                }).collect()
            }
        };

        
    });

    
    let mut posts_list = PostsListProps{
        posts : Vec::new(),
    };
    
    spawn_local(async { 
        update_posts(&mut posts_list).await;
    });

    html! {
        <main>           
            <h1>{ "Hello World!" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <PostsList ..posts_list />
            </div>
        </main>
    }
}*/

fn main() {
    // Init logger
    tracing_wasm::set_as_global_default();

    // Start WASM app
    yew::Renderer::<App>::new().render();
}
