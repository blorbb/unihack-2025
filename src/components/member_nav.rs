use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::components::A;

stylance::import_style!(s, "member_nav.module.scss");

#[component]
pub fn MemberNav() -> impl IntoView {
    mview! {
        ul class={s::member_nav} (
            li (A href="" ("Preferences"))
            li (A href="calendar" ("Calendar"))
      )
    }
}
