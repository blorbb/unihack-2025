use std::collections::BTreeSet;

use crate::components::Selector;
use leptos::prelude::*;
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

    let add_unit = move |unit| {
        member.write().units.push(MemberUnitPreferences {
            code: unit,
            activities: Default::default(),
        })
    };

    mview! {
        div class={s::page} (
            h1({member.read().name.clone()})

            input class={s::search_units_input}
                type="text"
                placeholder="Add unit"
                bind:value={query};
            ul class={s::searched_units} (
                Transition fallback=["Loading..."]
                (
                    [Suspend::new(async move { match units.await {
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
                        UnitPreferences {unit} {member} group_members={group.members.iter().map(|mem| mem.name.clone()).collect()};
                    )
                }
            )
        )
    }
    .into_any()
}

#[component]
fn UnitPreferences(
    unit: MemberUnitPreferences,
    member: RwSignal<MemberInfo>,
    group_members: BTreeSet<String>,
) -> impl IntoView {
    let unit = StoredValue::new(unit);
    let group_members = StoredValue::new(group_members);

    let set_activity_users = move |activity: String, members: BTreeSet<String>| {
        let mut member_guard = member.write();
        let Some(unit) = member_guard
            .units
            .iter_mut()
            .find(|u| u.code == unit.read_value().code)
        else {
            return;
        };
        unit.activities.insert(activity, members);
    };

    mview! {
        div class={s::unit_preferences} (
            h3({unit.read_value().code.clone()})

            table class={s::unit_table} (
                tr(
                    th("Activity")
                    th("Share with")
                )
                For each=[unit.get_value().activities]
                    key={|pref| pref.0.clone()}
                |(activity, members)| (
                    tr(
                        td({activity.clone()})
                        td(
                            Selector
                                options={Signal::derive(move || group_members.get_value())}
                                selected={Signal::derive(move || members.clone().into_iter().collect())}
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
