use backend::{activity::UnitCode, Member};
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
    let update_member = ServerAction::<api::UpdateMember>::new();

    let add_unit = move |unit, member: Member| {
        update_member.dispatch(api::UpdateMember {
            group_id: group(),
            member: member.tap_mut(|member| member.units.push(unit)),
        });
        get_group.dispatch(api::GetGroup { id: group() });
    };

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
                    let group = match group {
                        Err(_) => return Err(GetError::ServerError),
                        Ok(None) => return Err(GetError::GroupNotFound),
                        Ok(Some(group)) => group
                    };
                    let member = group.members.into_iter().find(|m| m.name == member()).ok_or(GetError::MemberNotFound)?;
                    let member2 = member.clone();
                    Ok(mview! {
                        Preferences
                            add_unit={move |unit| add_unit(unit, member.clone())}
                            member={member2};
                    }.into_any())
                })]
            )
        )
    }
}

#[component]
pub fn Preferences(
    add_unit: impl Fn(UnitCode) + 'static,
    #[prop(into)] member: Signal<Member>,
) -> impl IntoView {
    let add_unit_input = RwSignal::new(String::new());
    mview! {
        h1({member().name})

        h2 ("Units")

        ul (
            For
                each=[member().units.clone()]
                key={String::clone}
            |unit| {
                li ({unit})
            }
        )

        input type="text" placeholder="Add unit" bind:value={add_unit_input};
        Button variant={ButtonVariant::Primary} on:click={move |_| {
            logging::log!("{}", add_unit_input());

            add_unit(add_unit_input());
        }} ("+")

        h2 ("Preferences")
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
