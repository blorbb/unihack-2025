use backend::Member;
use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

use crate::api;

stylance::import_style!(s, "calendar.module.scss");

#[derive(Params, Clone, Default, PartialEq)]
struct CalendarParams {
    group: String,
    member: String,
}

#[component]
pub fn CalendarPage() -> impl IntoView {
    let param = use_params::<CalendarParams>();
    let member_resource = Resource::new(
        move || param.read().clone().unwrap_or_default(),
        |CalendarParams { group, member }| api::get_member(group, member),
    );

    mview! {
        Suspense
            fallback=[mview! { p("Loading user...") }]
        (
            ErrorBoundary
                fallback={|err| mview! { "Oops!" f["{:#?}", err()] }}
            (
                [Suspend::new(async move {
                    match member_resource.await {
                        Err(_) => Err(GetError::ServerError),
                        Ok(None) => Err(GetError::MemberNotFound),
                        Ok(Some(member)) => {
                            Ok(mview! {
                               Calendar member={member};
                            })
                        }
                    }
                })]
            )
        )
    }
}

#[component]
pub fn Calendar(#[prop(into)] member: Signal<Member>) -> impl IntoView {
    mview! {
        h1({member().name})


    }
}

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("User not found.")]
    MemberNotFound,
    #[error("Server error.")]
    ServerError,
}
