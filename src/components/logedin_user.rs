use yew::prelude::*;

use crate::services::blog_context::BlogContext;

fn short_npub(npub: &str) -> String {
    if npub.len() <= 8 {
        npub.to_string()
    } else {
        std::format!("{}...{}", &npub[..4], &npub[npub.len() - 4..])
    }
}

#[function_component(LogedinUser)]
pub fn logedin_user() -> Html {
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");

    let Some(login_info) = &*ctx.login_info else {
        return html! {};
    };

    let npub = login_info.pub_key.clone();
    let short_npub = short_npub(&npub);

    let copy_npub = {
        let npub = npub.clone();

        Callback::from(move |_| {
            let npub = npub.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    let _ = wasm_bindgen_futures::JsFuture::from(
                        window.navigator().clipboard().write_text(&npub)
                    )
                    .await;
                }
            });
        })
    };

    html! {
        <span class="login-display">
            {"As: "}

            {
                match (&login_info.display_name, &login_info.name) {
                    // Display name + name
                    (Some(display_name), Some(name)) => html! {
                        <>
                            {display_name}
                            {" ("}
                            {name}
                            {") ["}
                            {short_npub.clone()}
                            {"] "}
                            <button onclick={copy_npub.clone()}>
                                {"📋"}
                            </button>
                        </>
                    },

                    // Display name, but no name
                    (Some(display_name), None) => html! {
                        <>
                            {display_name}
                            {" ["}
                            {short_npub.clone()}
                            {"] "}
                            <button onclick={copy_npub.clone()}>
                                {"📋"}
                            </button>
                        </>
                    },

                    // Name, but no display name
                    (None, Some(name)) => html! {
                        <>
                            {name}
                            {" ["}
                            {short_npub.clone()}
                            {"] "}
                            <button onclick={copy_npub.clone()}>
                                {"📋"}
                            </button>
                        </>
                    },

                    // Neither name nor display name
                    (None, None) => html! {
                        {&login_info.pub_key}
                    },
                }
            }
        </span>
    }
}