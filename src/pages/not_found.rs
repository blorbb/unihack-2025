use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::components::A;

stylance::import_style!(s, "not_found.module.scss");

#[component]
pub fn NotFound() -> impl IntoView {
    mview! {
        main class={s::page_not_found} (
            h1("404 Page Not Found")
            p("Oops! How did you end up here?")
            p(A href="/" ("Go home?"))
        )
    }
}
