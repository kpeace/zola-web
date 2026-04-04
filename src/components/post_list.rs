use yew::prelude::*;
use crate::components::post_preview::PostPreview;
use crate::services::blog_context::BlogContext;
use gloo_console::log;


#[function_component(PostList)]
pub fn post_list() -> Html {
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");

    let cur_window = (*ctx.state).current_window;
    log!(format!("Rendering window: {}", cur_window));

    let ctx_clone = ctx.clone();
    let on_click_next = Callback::from(move |_| {
        let mut state = (*ctx_clone.state).clone();
        state.current_window += 1;

        let async_ctx_clone = ctx_clone.clone();
        let next_window = state.current_window + 1;
        wasm_bindgen_futures::spawn_local(async move {
            async_ctx_clone.load_window(next_window).await;
        });

        ctx_clone.state.set(state);
    });

    let ctx_clone = ctx.clone();
    let on_click_prev = Callback::from(move |_| {
        let mut state = (*ctx_clone.state).clone();

        if state.current_window == 0 {
            return;
        }

        state.current_window -= 1;
        ctx_clone.state.set(state);
    });




    let posts = {
        let guard = ctx.windows.read().expect("RwLock poisoned");

        guard
            .get(&cur_window)
            .filter(|_| cur_window >= 0 && !guard.is_empty())
            .map(|post_window| post_window.posts.clone())
    };


    match posts {
        Some(posts) => {
            html! {
                <div class="post-list">
                    { for posts.iter().map(|p| html!{
                        <PostPreview post={p.clone()} />
                    })}
                    <button onclick={on_click_prev}>{ "Prev" }</button>
                    <button onclick={on_click_next}>{ "Next" }</button>
                </div>
             }
        }
        None => {
            html! {
                <div>{ "Loading posts..." }</div>
            }
        }
    }
}