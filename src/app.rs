use yew::prelude::*;
use yew_router::prelude::*;
use std::rc::Rc;
use crate::router::{switch, Route};
use crate::nostr::client::NostrClient;
use crate::services::blog_post::BlogPost;
use crate::services::blog_context::{BlogContext, BlogState, new_window_cache};
use crate::services::config::{AppConfig, ConfigError};
use gloo_console::log;
use crate::components::page_layout::PageLayout;

#[derive(Clone, PartialEq)]
pub enum BlogContextState {
    Loading,
    Ready(BlogContext),
}

#[hook]
fn use_blog_context(config_opt: Option<Rc<AppConfig>>) -> Option<BlogContext> {

    let state = use_state(|| BlogState {
        current_window: -1,
    });

    let windows = use_memo((), |_| new_window_cache());

    let client = use_state(|| None::<Rc<NostrClient>>);

    // init client
    {
        let client = client.clone();
        let config_opt = config_opt.clone();

        use_effect_with(config_opt.clone(), move |cfg| {
            if let Some(cfg) = cfg {
                let c = Rc::new(
                    NostrClient::new(&cfg.public_key)
                        .expect("Failed to initialize NostrClient")
                );
                client.set(Some(c));
            }
            || ()
        });
    }

    // load data
    {
        let client = client.clone();
        let state = state.clone();
        let windows = (*windows).clone();
        let config_opt = config_opt.clone();

        use_effect_with(client.clone(), move |client_handle| {
            // extract + CLONE outside async
            //let client = client_handle.as_ref().cloned();
            //let config = config_opt.as_ref().cloned();
            if let (Some(client), Some(config)) = (client_handle.as_ref(), config_opt.as_ref()) {
                let context = BlogContext {
                    client: client.clone(),
                    state: state.clone(),
                    windows: windows.clone(),
                    config: config.clone(),
                };

                wasm_bindgen_futures::spawn_local(async move {
                    let (tx, mut rx) = futures::channel::mpsc::unbounded::<BlogPost>();

                    context.client.connect().await.unwrap();

                    context.load_window(0).await;
                    context.load_window(1).await;

                    state.set(BlogState { current_window: 0 });

                    let newest = context.get_newest().unwrap_or_default();

                    let client_clone = context.client.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        client_clone.subscribe_posts(newest, tx).await;
                    });

                    use futures::StreamExt;
                    while let Some(post) = rx.next().await {
                        context.add_post(post);
                    }
                });
            }

            || ()
        });
    }

    // 👇 FINAL VALUE (NO early return)
    match ((*client).clone(), config_opt.clone()) {
        (Some(client), Some(config)) => Some(BlogContext {
            client,
            state,
            windows: (*windows).clone(),
            config,
        }),
        _ => None,
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let config = use_state(|| None::<Rc<AppConfig>>);
    let error = use_state(|| None::<ConfigError>);

    // Load config once
    {
        let config = config.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match AppConfig::load().await {
                    Ok(cfg) => config.set(Some(Rc::new(cfg))),
                    Err(err) => {
                        log!(format!("Config load failed: {}", err));
                        error.set(Some(err));
                    }
                }
            });
            || ()
        });
    }

     let ctx = use_blog_context((*config).clone());

    if let Some(err) = (*error).as_ref() {
        return html! {
            <div class="error">
                <h2>{"Failed to load configuration"}</h2>
                <pre>{ format!("{}", err)}</pre>
            </div>
        };
    }

    if ctx.is_none() {
        return html! {
            <div class="loading">
                <h2>{"Loading blog configuration..."}</h2>
                <p>{"Please wait"}</p>
            </div>
        };
    }

    let ctx = ctx.unwrap();

    html! {
        <ContextProvider<BlogContext> context={ctx}>
            <BrowserRouter>
                <PageLayout>
                    <Switch<Route> render={switch} />
                </PageLayout>
            </BrowserRouter>
        </ContextProvider<BlogContext>>
    }
}