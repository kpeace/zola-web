use yew::prelude::*;
use crate::services::blog_context::BlogContext;

#[derive(Properties, PartialEq)]
pub struct LoginButtonProps {
    pub on_login: Callback<MouseEvent>,
}

#[function_component(LoginButton)]
pub fn login_button(props: &LoginButtonProps) -> Html {
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");
    let ctx_copy_npub_clone = ctx.clone();

    let menu_open = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();

        Callback::from(move |_| {
            menu_open.set(!*menu_open);
        })
    };

    // copy the users npub
    // this can be called only when we have a logged in user
    let copy_npub = {
        let ctx = ctx_copy_npub_clone.clone();
        let menu_open = menu_open.clone();


        Callback::from(move |_| {
            let login_info = ctx.login_info.as_ref().unwrap();
            let npub = login_info.pub_key.clone();
            let menu_open = menu_open.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    let _ = wasm_bindgen_futures::JsFuture::from(
                        window.navigator().clipboard().write_text(&npub.to_hex())
                    )
                    .await;
                    menu_open.set(!*menu_open);
                }
            });
        })
    };

    let log_off = {
        let ctx_clone = ctx.clone();
        Callback::from(move |_| {
            let ctx = ctx_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                ctx.log_out().await;
            });
        })
    };

    // Not logged in
    if ctx.login_info.is_none() {
        return html! {
            <button
                class="login-button"
                onclick={props.on_login.clone()}
            >
                <span class="login-icon">{"👤"}</span>
                <span>{"Login"}</span>
            </button>
        };
    }

    let login_info = ctx.login_info.as_ref().unwrap();

    let display_name = login_info
        .display_name
        .as_deref()
        .or(login_info.name.as_deref())
        .unwrap_or("Nostr user");

    let short_npub = login_info.short_pub_key();

    let picture = login_info.picture.clone();

    html! {
        <div class="profile-container">
            <button
                class="profile-chip"
                onclick={toggle_menu}
            >
                {
                    if let Some(picture) = picture {
                        html! {
                            <img
                                class="profile-avatar"
                                src={picture}
                                alt="Profile picture"
                            />
                        }
                    } else {
                        html! {
                            <div class="profile-avatar profile-avatar-placeholder">
                                {"👤"}
                            </div>
                        }
                    }
                }

                <span class="profile-info">
                    <span class="profile-name">
                        { display_name }
                    </span>

                    <span class="profile-npub">
                        { short_npub }
                    </span>
                </span>

                <span class="profile-arrow">
                    {"⌄"}
                </span>
            </button>

            {
                if *menu_open {
                    html! {
                        <div class="profile-menu">
                            <div class="profile-menu-npub">
                                { &login_info.pub_key.to_hex() }
                            </div>

                            <button class="profile-menu-item"
                                onclick={copy_npub.clone()}
                            >
                                {"Copy npub"}
                            </button>

                            <button class="profile-menu-item"
                                onclick={log_off}
                            >
                                {"Logout"}
                            </button>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}