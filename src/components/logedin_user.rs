use yew::prelude::*;

use crate::services::blog_context::BlogContext;

#[function_component(LogedinUser)]
pub fn logedin_user() -> Html {
    let ctx = use_context::<BlogContext>()
        .expect("BlogContext not found");

    if let Some(login_info) = &*ctx.login_info {
        html! {
            <span class="login-display">
                {"As: "}
                if let Some(display_name) = &login_info.display_name {
                    {display_name}
                } else {
                    {&login_info.pub_key}
                }
            </span>
        }
    } else {
        html! {}
    }
}
