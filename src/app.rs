use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;
use std::rc::Rc;
use crate::router::{switch, Route};
use crate::nostr::client::NostrClient;
use crate::services::blog_post::BlogPost;
use crate::services::blog_context::{BlogContext, BlogState, new_window_cache};
use futures::StreamExt;
use gloo_console::log;



#[hook]
fn use_blog_context() -> BlogContext {
    let state = use_state(|| BlogState {
        current_window : -1,
    });

    log!("Initializing Nostr client");

    let client = use_state(|| Rc::new(
        NostrClient::new().expect("Failed to initialize NostrClient")
    ));

    let windows = use_memo((), |_| new_window_cache());

    let context = BlogContext {
        client: (*client).clone(),
        state,
        windows: (*windows).clone(),
    };

    {
        let context = BlogContext {
            client: context.client.clone(),
            state: context.state.clone(),
            windows: context.windows.clone()
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

                wasm_bindgen_futures::spawn_local(async move {
                    client_clone.subscribe_posts(tx).await;
                });

                // TODO add cache window update here
                // while let Some(event) = rx.next().await {
                //     let current_state = (*state).clone(); // re-read latest state
                //     let mut posts = current_state.posts.as_ref().clone();

                //     posts.push(event);

                //     state.set(BlogState {
                //         posts: Rc::new(posts),
                //         oldest_timestamp:None
                //     });
                // }
            });

            || ()
        });
    }

    context
}

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_blog_context();

    html! {
        <ContextProvider<BlogContext> context={ctx}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<BlogContext>>
    }
}