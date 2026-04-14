use yew::prelude::*;
use yew_router::prelude::*;
use std::rc::Rc;
use crate::router::{switch, Route};
use crate::nostr::client::NostrClient;
use crate::services::blog_post::BlogPost;
use crate::services::blog_context::{BlogContext, BlogState, new_window_cache};
use crate::services::config::AppConfig;
use futures::StreamExt;
use gloo_console::log;



#[hook]
fn use_blog_context() -> Option<BlogContext> {
    let config = use_state(|| None::<Rc<AppConfig>>);   // None = loading

    {
        let config = config.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match AppConfig::load().await {
                    Ok(cfg) => {
                        config.set(Some(Rc::new(cfg)));
                    }
                    Err(err) => {
                        log!(format!("Config load failed: {}", err));
                    }
                }
            });
            || ()
        });
    }

    // 2. Only initialize the rest AFTER config is loaded
    let config_ref = (*config).clone();
    if config_ref.is_none() {
        return None;        // Still loading → return None
    }

    let state = use_state(|| BlogState {
        current_window : -1,
    });

    log!("Initializing Nostr client");

    // unwrap is safe since we checked that it is not none
    let client = use_state(|| Rc::new(
        NostrClient::new(&config_ref.unwrap().public_key).expect("Failed to initialize NostrClient")
    ));

    let windows = use_memo((), |_| new_window_cache());

    let context = BlogContext {
        client: (*client).clone(),
        state,
        windows: (*windows).clone(),
        config,
    };

    {
        let context = BlogContext {
            client: context.client.clone(),
            state: context.state.clone(),
            windows: context.windows.clone(),
            config: context.config.clone(),
        };

        use_effect_with((), move |_| {

            let (tx, mut rx) = futures::channel::mpsc::unbounded::<BlogPost>();

            wasm_bindgen_futures::spawn_local(async move {

                context.client.connect().await
                    .map_err(|e| {
                        log!(format!("Failed to connect to relay: {:?}", e));
                        e
                    })
                    .unwrap();

                // get initial window
                context.load_window(0).await;
                context.load_window(1).await;
                context.state.set(BlogState { current_window: 0 });

                let client_clone = client.clone();
                let newest = context.get_newest().unwrap_or_default();

                // DEBUG
                log!(format!("Newest blog: {}", newest));

                wasm_bindgen_futures::spawn_local(async move {
                    client_clone.subscribe_posts(newest, tx,).await;
                });

                // TODO add cache window update here
                while let Some(post) = rx.next().await {
                    log!(format!("Got new post: {} with time: {}", post.title, post.created_at));
                    context.add_post(post);
                }
            });

            || ()
        });
    }

    Some(context)
}

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_blog_context();

    if let Some(ctx) = ctx {
        html! {
            <ContextProvider<BlogContext> context={ctx}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<BlogContext>>
        }
    } else {
        // Loading screen while config is being fetched
        html! {
            <div class="loading">
                <h2>{"Loading blog configuration..."}</h2>
                <p>{"Please wait"}</p>
            </div>
        }
    }
}