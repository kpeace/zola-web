use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use gloo_console::log;
use crate::services::blog_context::BlogContext;

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

    html! {
        <div class="app-container">
            // Top banner
            <header class="header">
                <h1>{ title }</h1>
                <nav>
                    <Link<Route> to={Route::Home}>{ "Posts" }</Link<Route>>
                </nav>
            </header>

            // Page content
            <main class="content">
                { for props.children.iter() }
            </main>
        </div>
    }
}