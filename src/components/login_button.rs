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

    let menu_open = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();

        Callback::from(move |_| {
            menu_open.set(!*menu_open);
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
                                { &login_info.pub_key }
                            </div>

                            <button class="profile-menu-item">
                                {"Copy npub"}
                            </button>

                            <button class="profile-menu-item">
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