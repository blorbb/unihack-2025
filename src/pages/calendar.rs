use std::collections::BTreeMap;

use backend::activity::{Class, WeekDay};
use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::api;

stylance::import_style!(s, "calendar.module.scss");

/// 8 am
const START_TIME: u16 = 8 * 60;
/// 10 pm
const END_TIME: u16 = 20 * 60;

#[derive(Params, Clone, Default, PartialEq)]
struct CalendarParams {
    group: String,
    member: String,
}

#[component]
pub fn CalendarPage() -> impl IntoView {
    let param = use_params::<CalendarParams>();
    let calendar_resource = Resource::new(
        move || param.read().clone().unwrap_or_default(),
        |CalendarParams { group, member }| api::get_member_calendar(group, member),
    );

    mview! {
        Suspense
            fallback=[mview! { p("Loading user...") }]
        (
            ErrorBoundary
                fallback={|err| mview! { "Oops!" f["{:#?}", err()] }}
            (
                [Suspend::new(async move {
                    match calendar_resource.await {
                        Err(e) => Err(GetError::ServerError(e)),
                        Ok(calendar) => {
                            Ok(mview! {
                            h1({param.read().as_ref().map(|x| x.member.clone()).unwrap_or_default()})
                               Calendar calendar={calendar};
                            })
                        }
                    }
                })]
            )
        )
    }
}

#[component]
pub fn Calendar(
    #[prop(into)] calendar: Signal<BTreeMap<String, BTreeMap<String, Class>>>,
) -> impl IntoView {
    mview! {
        div class={s::calendar} (
            For
                each=[WeekDay::iter()]
                key={WeekDay::clone}
            |day| {
                div (
                    h2 class={s::day} ({<&str>::from(day)})

                    div class={s::day_schedule} ()
                )
            }
        )
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("User not found.")]
    MemberNotFound,
    #[error("{0}")]
    ServerError(ServerFnError),
}
