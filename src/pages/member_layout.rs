use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::{
    components::{Outlet, A},
    hooks::use_params,
    params::Params,
};

stylance::import_style!(s, "member_layout.module.scss");

#[derive(Params, Clone, Default, PartialEq)]
struct MemberParams {
    member: String,
}

#[component]
pub fn MemberLayout() -> impl IntoView {
    let param = use_params::<MemberParams>();
    let member = move || {
        param
            .read()
            .as_ref()
            .map(|params| params.member.clone())
            .unwrap_or_default()
    };

    mview! {
        div(
            nav class={s::layout} (
                h1 ({member()})
                ul class={s::member_nav} (
                    li (A href="" ("Preferences"))
                    li (A href="timetable" ("Timetable"))
                )
            )
            Outlet;
        )
    }
}
