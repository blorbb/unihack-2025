use leptos::prelude::*;
use leptos_mview::mview;

use crate::components::{button::ButtonVariant, Button};

stylance::import_style!(s, "home.module.scss");

#[component]
pub fn HomePage() -> impl IntoView {
    let create_group = ServerAction::<CreateGroup>::new();
    let join_group = ServerAction::<JoinGroup>::new();

    mview! {
        main class={s::page_home} (
            h2 class={s::heading_create} ("Create new group")
            div class={s::form_create} (
                ActionForm action={create_group} (
                    Button
                        variant={ButtonVariant::Primary}
                        class={s::create_group}
                        type="submit"
                    (
                       "+"
                    )
                )
            )
            h2 class={s::heading_join} ("Join group")
            div class={s::form_join} (
                ActionForm action={join_group} (
                    input class={s::join_input}
                        type="search" name="group" placeholder="Group ID";
                    Button
                        variant={ButtonVariant::Primary}
                        class={s::join_button}
                        type="submit"
                    (
                        "Join"
                    )
                )
            )
        )
    }
}

#[server]
async fn create_group() -> Result<(), ServerFnError> {
    let group_id = urlencoding::encode("testid");
    leptos_axum::redirect(&format!("/g/{group_id}"));
    Ok(())
}

#[server]
async fn join_group(group: String) -> Result<(), ServerFnError> {
    leptos_axum::redirect(&format!("/g/{}", urlencoding::encode(group.trim())));
    Ok(())
}
