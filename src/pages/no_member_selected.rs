use leptos::prelude::*;
use leptos_mview::mview;

stylance::import_style!(s, "no_member_selected.module.scss");

#[component]
pub fn NoMemberSelected() -> impl IntoView {
    mview! {
        div class={s::layout} (
            // TODO: add member button
            h2 class={s::no_member} ("Select or add a new group member")
        )
    }
}
