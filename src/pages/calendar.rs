use backend::{activity::WeekDay, Member};
use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

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
        async |CalendarParams { group, member }| {
            api::get_group(group)
                .await
                .map(|group| group?.members.into_iter().find(|mem| mem.name == member))
        },
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

        div (
            For
                each=[WeekDay::iter()]
                key={WeekDay::clone}
            |day| {
                div (
                    h2({<&str>::from(day)})
                )
            }
        )
    }
}

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("User not found.")]
    MemberNotFound,
    #[error("Server error.")]
    ServerError,
}
