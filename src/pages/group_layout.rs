use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::{
    components::{Outlet, A},
    hooks::use_params,
    params::Params,
};
use serde::{Deserialize, Serialize};

use crate::api;

stylance::import_style!(s, "group_layout.module.scss");

#[derive(Params, PartialEq)]
struct GroupParams {
    group: String,
}

#[component]
pub fn GroupLayout() -> impl IntoView {
    let param = use_params::<GroupParams>();
    let group = move || {
        param
            .read()
            .as_ref()
            .ok()
            .map(|params| params.group.clone())
            .unwrap_or_default()
    };
    let group_resource = Resource::new(group, api::get_group);

    mview! {
        Suspense
            fallback=[mview! { p("Loading group...") }]
        (
            ErrorBoundary
                fallback={|err| mview! { "Oops!" f["{:#?}", err()] }}
            (
                [Suspend::new(async move {
                    let group = group_resource.await;
                    let view = match group {
                        Ok(Some(g)) => mview! {
                            div class={s::layout} (
                                GroupList group={g};
                                main(Outlet;)
                            )
                        },
                        Ok(None) => return Err(GetError::GroupNotFound),
                        Err(_) => return Err(GetError::ServerError)
                    };
                    Ok(view)
                })]
            )
        )
    }
}

#[component]
fn GroupList(#[prop(into)] group: Signal<backend::Group>) -> impl IntoView {
    mview! {
        nav class={s::member_list_wrapper} (
            h1 class={s::home_link} (A href="/" ("Una"))

            h2("Group Members")
            ul (
                For
                    each=[group.read().members.iter().map(|m| m.name.clone()).collect::<Vec<_>>()]
                    key={|member| member.clone()}
                |member| {
                    li class={s::member} (
                        A attr:class={s::member_link} href={urlencoding::encode(&member).into_owned()} (
                            span class={s::member_name} ({member})
                            span class={s::member_units} ("TODO")
                        )
                    )
                }
            )

        )
    }
}

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("Invalid group ID.")]
    InvalidId,
    #[error("Group not found.")]
    GroupNotFound,
    #[error("Server error.")]
    ServerError,
}
