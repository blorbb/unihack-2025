use std::collections::{BTreeMap, HashMap};

use backend::activity::{Activity, Class, UnitCode, WeekDay};
use itertools::Itertools;
use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::api;

stylance::import_style!(s, "timetable.module.scss");

/// 8 am
const START_TIME: u16 = 8 * 60;
/// 10 pm
const END_TIME: u16 = 22 * 60;

fn format_time(time: u16) -> String {
    let hrs = time / 60;
    let mins = time % 60;
    format!("{hrs:02}:{mins:02}")
}

// fn activity_to_colour(activity: Activity) -> &'static str {
//     [
//         ("Applied", ""),
//         ("Studio", ""),
//         ("Tutorial", ""),
//         //
//         ("Practical", ""),
//         ("Laboratory", ""),
//         //
//         ("Lecture", ""),
//         ("Workshop", ""),
//         ("Seminar", ""),
//     ]
//     .into_iter()
//     .find(|(x, _)| activity.starts_with(x))
//     .map(|(_, colour)| colour)
//     .unwrap_or("")
// }

#[derive(Params, Clone, Default, PartialEq)]
struct TimetableParams {
    group: String,
    member: String,
}

#[component]
pub fn TimetablePage() -> impl IntoView {
    let param = use_params::<TimetableParams>();
    let timetable_resource = Resource::new(
        move || param.read().clone().unwrap_or_default(),
        |TimetableParams { group, member }| api::get_member_calendar(group, member),
    );

    mview! {
        Suspense
            fallback=[mview! { p("Loading user...") }]
        (
            ErrorBoundary
                fallback={|err| mview! { "Oops!" f["{:#?}", err()] }}
            (
                [Suspend::new(async move {
                    match timetable_resource.await {
                        Err(e) => Err(GetError::ServerError(e)),
                        Ok(timetable) => {
                            Ok(mview! {
                               Timetable timetable={timetable};
                            })
                        }
                    }
                })]
            )
        )
    }
}

const COLOURS: [&str; 5] = ["blue", "green", "orange", "red", "grape"];
const DEFAULT_COLOUR: &str = "pink";

#[component]
fn Timetable(timetable: BTreeMap<UnitCode, BTreeMap<Activity, Class>>) -> impl IntoView {
    let units_to_colour = StoredValue::new(
        timetable
            .keys()
            .enumerate()
            .map(|(i, unit)| {
                (
                    unit.clone(),
                    COLOURS.get(i).cloned().unwrap_or(DEFAULT_COLOUR),
                )
            })
            .collect::<HashMap<_, _>>(),
    );
    let classes = StoredValue::new(
        timetable
            .into_iter()
            .flat_map(|(unit, classes)| {
                classes
                    .into_iter()
                    .map(move |(activity, class)| (unit.clone(), activity, class))
            })
            .into_group_map_by(|(_, _, class)| class.day),
    );

    mview! {
        div class={s::timetable} (
            For
                each=[WeekDay::iter()]
                key={WeekDay::clone}
            |day| {
                div class={s::day} (
                    h2 class={s::day_header} ({<&str>::from(day)})

                    div class={s::day_schedule} (
                        For
                            each=[classes.read_value().get(&day).cloned().unwrap_or_default()]
                            key={|(unit, activity, _)| (unit.clone(), activity.clone())}
                        |class| {
                            Class units_to_colour={units_to_colour.get_value()} class={class};
                        }
                    )
                )
            }
        )
    }
}

#[component]
pub fn Class(
    units_to_colour: HashMap<UnitCode, &'static str>,
    #[prop(name = "class")] (unit, activity, class): (UnitCode, Activity, Class),
) -> impl IntoView {
    let top = (class.start - START_TIME) as f32 / (END_TIME - START_TIME) as f32 * 100.0;
    let height = (class.end - class.start) as f32 / (END_TIME - START_TIME) as f32 * 100.0;
    let colour = *units_to_colour.get(&unit).unwrap();
    mview! {
        div class={s::class} style:top=f["{}%", top] style:height=f["{}%", height] (
            div class={s::class_inner} style:background-color=f["var(--{}-3)", colour] (
                p (strong ({format!("{unit} {activity}")}))
                p ({class.code})
                p ({format!("{} – {}", format_time(class.start), format_time(class.end))})
            )
        )
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum GetError {
    #[error("User not found.")]
    MemberNotFound,
    #[error("{0}")]
    ServerError(ServerFnError),
}
