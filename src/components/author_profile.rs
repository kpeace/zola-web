use yew::prelude::*;
use crate::services::{blog_context::BlogContext, login_info::{LoginInfo}};
use crate::services::public_key::PublicKey;
use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub pub_key: PublicKey,
}

#[function_component(AuthorProfile)]
pub fn author_profile(props: &Props) -> Html {
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");

    // set initial profile info using what we have
    let profile = use_state(||
        LoginInfo {
            pub_key: props.pub_key.clone(),
            ..Default::default()
        }
    );

    // in the background load the rest of the profile
    let ctx_clone = ctx.clone();
    let profile_clone = profile.clone();

    use_effect_with(
    props.pub_key.clone(),
    move |pub_key| {
        let ctx = ctx_clone.clone();
        let profile = profile_clone.clone();
        let pub_key = pub_key.clone();

        wasm_bindgen_futures::spawn_local(async move {
            log!(format!("Fetching profile for: {}", pub_key.to_hex()));

            profile.set(ctx.get_profile(&pub_key).await);
        });
    },
);

    let short_npub = profile.short_pub_key();

    let copy_npub = {
        let npub = profile.pub_key.clone();

        Callback::from(move |_| {
            let npub = npub.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    let _ = wasm_bindgen_futures::JsFuture::from(
                        window.navigator().clipboard().write_text(&npub.to_hex())
                    )
                    .await;
                }
            });
        })
    };




    html! {
        <span class="login-display">

            {
                match (&profile.display_name, &profile.name) {
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
                        {&profile.pub_key.to_hex()}
                    },
                }
            }
        </span>
    }

}