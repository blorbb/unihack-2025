use std::collections::BTreeMap;

use backend::activity::{Activity, Class, UnitCode, WeekDay};
use itertools::Itertools;
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

// TODO: rename to Timetable
#[component]
pub fn CalendarPage() -> impl IntoView {
    let param = use_params::<CalendarParams>();
    let member = move || {
        param
            .read()
            .as_ref()
            .map(|params| params.member.clone())
            .unwrap_or_default()
    };
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
                            h1({format!("{}’s Timetable", member())})
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
pub fn Calendar(calendar: BTreeMap<UnitCode, BTreeMap<Activity, Class>>) -> impl IntoView {
    let classes = StoredValue::new(
        calendar
            .into_iter()
            .flat_map(|(unit, classes)| {
                classes
                    .into_iter()
                    .map(move |(activity, class)| (unit.clone(), activity, class))
            })
            .into_group_map_by(|(_, _, class)| class.day),
    );

    mview! {
        div class={s::calendar} (
            For
                each=[WeekDay::iter()]
                key={WeekDay::clone}
            |day| {
                div (
                    h2 class={s::day} ({<&str>::from(day)})

                    div class={s::day_schedule} (
                        For
                            each=[classes.read_value().get(&day).cloned().unwrap_or_default()]
                            key={|(unit, activity, _)| (unit.clone(), activity.clone())}
                        |class| {
                            Class class={class};
                        }
                    )
                )
            }
        )
    }
}

#[component]
pub fn Class(
    #[prop(name = "class")] (unit, activity, class): (UnitCode, Activity, Class),
) -> impl IntoView {
    mview! {
        div (
            p ({format!("{unit} {activity}: {class:?}")})
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
