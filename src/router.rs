use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::post_list::PostList;
use crate::components::post_page::PostPage;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/post/:slug")]
    Post { slug: String },

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <PostList /> },

        Route::Post { slug } => html! {
            <PostPage slug={slug}/>
        },

        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}