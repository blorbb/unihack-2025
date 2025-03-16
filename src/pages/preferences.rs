use std::collections::{BTreeMap, BTreeSet};

use crate::{
    api::update_member,
    clone_in,
    components::{button::ButtonVariant, Button, Selector},
};
use leptos::{prelude::*, task::spawn_local};
use leptos_mview::mview;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};

use crate::api::{self, GroupInfo, MemberInfo, MemberUnitPreferences};

stylance::import_style!(s, "preferences.module.scss");

#[derive(Params, Clone, Default, PartialEq)]
struct PreferencesParams {
    group: String,
    member: String,
}

#[component]
pub fn PreferencesPage() -> impl IntoView {
    let param = use_params::<PreferencesParams>();
    let group = move || {
        param
            .read()
            .as_ref()
            .map(|params| params.group.clone())
            .unwrap_or_default()
    };
    let member = move || {
        param
            .read()
            .as_ref()
            .map(|params| params.member.clone())
            .unwrap_or_default()
    };

    let get_group = ServerAction::<api::GetGroup>::new();

    // refresh member preferences on every group/member change
    Effect::new(move || {
        get_group.dispatch(api::GetGroup { id: group() });
    });

    mview! {
        Suspense
            fallback=[mview! { p("Loading user...") }]
        (
            ErrorBoundary
                fallback={|err| mview! { "Oops!" f["{:#?}", err()] }}
            (
                [Suspend::new(async move {
                    let Some(group) = get_group.value()() else {
                        return Ok(mview! {
                            "Loading..."
                        }.into_any())
                    };
                    match group {
                        Err(_) => Err(GetError::ServerError),
                        Ok(None) => Err(GetError::MemberNotFound),
                        Ok(Some(group)) => {
                            Ok(mview! {
                                Preferences
                                    group={group}
                                    member={member()};
                            }.into_any())
                        }
                    }
                })]
            )
        )
    }
}

#[component]
pub fn Preferences(
    #[prop(into)] group: GroupInfo,
    /// Member MUST be in `group`
    #[prop(into)]
    member: String,
) -> impl IntoView {
    let Some(member) = group.members.iter().find(|mem| mem.name == member).cloned() else {
        return mview! {
            "Member not found"
        }
        .into_any();
    };
    let member = RwSignal::new(member);

    let query = RwSignal::new(String::new());
    let units = Resource::new(query, api::search_units);

    let add_unit = move |unit: String| {
        leptos::task::spawn_local(async move {
            let Ok(Some(activities)) = api::get_unit_activities(unit.clone()).await else {
                return;
            };
            member.write().units.insert(
                0,
                MemberUnitPreferences {
                    code: unit,
                    activities: BTreeMap::from_iter(
                        activities.into_iter().map(|act| (act, BTreeSet::default())),
                    ),
                },
            )
        });
    };

    mview! {
        div class={s::page} (
            input class={s::search_units_input}
                type="text"
                placeholder="Add unit"
                bind:value={query};

            ul class={s::searched_units} (
                Transition fallback=[mview! {
                    div class={s::searched_unit} ("Loading...")
                }]
                (
                    [Suspend::new(async move { match units.await {
                        _ if query().trim().is_empty() => mview! {
                            div class={s::searched_unit} ("Start typing to search")
                        }.into_any(),
                        // TODO: what if unit is already selected so it's filtered out and it looks empty
                        Ok(units) if units.is_empty() => mview! {
                            div class={s::searched_unit} ("No units found")
                        }.into_any(),
                        Ok(units) => mview! {
                            For
                                each=[
                                    units.iter()
                                        .filter(|unit| !member.read().units.iter().any(|u| u.code == **unit))
                                        .cloned()
                                        .collect::<Vec<_>>()
                                ]
                                key={String::clone}
                            |unit| {
                                li(
                                    button class={s::searched_unit}
                                        on:click={
                                            let unit = unit.clone();
                                            move |_| add_unit(unit.clone())
                                        }
                                    ({unit})
                                )
                            }
                        }.into_any(),
                        Err(e) => format!("Oops, something went wrong.\n{e}").into_any()
                    }})]
                )
            )

            h2("Selected Units")

            ul class={s::preferences} (
                For
                    each=[member.read().units.clone()]
                    key={|unit| unit.code.clone()}
                |unit| {
                    li(
                        UnitPreferences unit={unit.code} {member} group_members={group.members.iter().map(|mem| mem.name.clone()).collect()};
                    )
                }
            )

            Button
                variant={ButtonVariant::Primary}
                on:click={move |_| {
                    let group_id = group.id.clone();
                    spawn_local(async move {
                        update_member(group_id, member()).await.expect("update member failed")
                    })
                }}
                ("Submit")
        )
    }
    .into_any()
}

#[component]
fn UnitPreferences(
    unit: String,
    member: RwSignal<MemberInfo>,
    group_members: BTreeSet<String>,
) -> impl IntoView {
    let unit_code = StoredValue::new(unit);
    let unit = move || {
        leptos::logging::log!("getting");
        member
            .read()
            .units
            .iter()
            .find(|u| u.code == *unit_code.read_value())
            .unwrap()
            .clone()
    };
    let group_members = StoredValue::new(group_members);

    let set_activity_users = move |activity: String, members: BTreeSet<String>| {
        leptos::logging::log!("{members:?}");
        let mut member_guard = member.write();
        let Some(unit) = member_guard
            .units
            .iter_mut()
            .find(|u| u.code == *unit_code.read_value())
        else {
            return;
        };
        unit.activities.insert(activity, members);
    };

    mview! {
        div class={s::unit_preferences} (
            h3({unit_code.get_value()})

            table class={s::unit_table} (
                tr(
                    th("Activity")
                    th("Share with")
                )
                For each=[unit().activities]
                    key={|pref| pref.0.clone()}
                |(activity, _members)| (
                    tr(
                        td({activity.clone()})
                        td(
                            Selector
                                options={Signal::derive(move || group_members.get_value())}
                                // members needs to be accessed through unit() to be reactive
                                selected={Signal::derive(clone_in!(activity, move || unit().activities[&activity].clone()))}
                                set_selected={move |sel| set_activity_users(activity.clone(), sel)};
                        )
                    )
                )
            )
        )
    }
}

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("Group not found.")]
    GroupNotFound,
    #[error("User not found.")]
    MemberNotFound,
    #[error("Server error.")]
    ServerError,
}
