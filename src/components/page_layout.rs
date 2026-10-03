use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use gloo_console::log;
use crate::services::blog_context::BlogContext;
use crate::components::login_button::LoginButton;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(PageLayout)]
pub fn layout(props: &LayoutProps) -> Html {
    log!("Rendering layout");
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");

    let title: String = ctx.config.title.clone();

    let on_login = {

        Callback::from(move|_| {
            let ctx_clone = ctx.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(login_info) = ctx_clone.login().await {
                    log!(format!("got login! npub {}", login_info.pub_key));
                    ctx_clone.login_info.set(Some(login_info));
                    if let Some(ctx_login_info) = &*ctx_clone.login_info {
                        log!(format!(
                            "login_info set to: {}",
                            ctx_login_info.clone().pub_key
                        ));
                    }

                }
            });

        })
    };

    html! {
    <div class="app-container">
        <header class="header">
            <div class="header-left">
                <h1>{ title }</h1>

                <nav>
                    <Link<Route> to={Route::Home}>
                        { "Posts" }
                    </Link<Route>>
                </nav>
            </div>

            <LoginButton on_login={on_login} />
        </header>

        <main class="content">
            { for props.children.iter() }
        </main>
    </div>
}
}