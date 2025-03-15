use backend::{activity::UnitCode, Group, Member};
use leptos::{logging, prelude::*};
use leptos_mview::mview;
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};
use tap::Tap;

use crate::{
    api,
    components::{button::ButtonVariant, Button},
};

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
    #[prop(into)] group: Group,
    /// Member MUST be in `group`, otherwise this will panic.
    #[prop(into)]
    member: String,
) -> impl IntoView {
    let Some(member) = group.members.into_iter().find(|mem| mem.name == member) else {
        return mview! {
            "Member not found"
        }
        .into_any();
    };
    let member = RwSignal::new(member);

    let query = RwSignal::new(String::new());
    let units = Resource::new(query, api::search_units);

    let add_unit = move |unit| {};

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
                    [Suspend::new(async move {
                        match units.await {
                            Ok(units) => mview! {
                                For
                                    each=[
                                        units.iter()
                                            .filter(|unit| !member.read().units.contains(unit))
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
                        }
                    })]
                )
            )

            ul (
                For
                    each=[member.read().units.clone()]
                    key={String::clone}
                |unit| {
                    li ({unit})
                }
            )


            h2 ("Preferences")
        )
    }
    .into_any()
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
