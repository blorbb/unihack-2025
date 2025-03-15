use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::components::{Outlet, A};

stylance::import_style!(s, "member_layout.module.scss");

#[component]
pub fn MemberLayout() -> impl IntoView {
    mview! {
        ul class={s::member_nav} (
            li (A href="" ("Preferences"))
            li (A href="calendar" ("Calendar"))
        )
        Outlet;
    }
}
