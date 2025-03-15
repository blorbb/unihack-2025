use backend::{activity::UnitCode, Member};
use leptos::{logging, prelude::*};
use leptos_mview::mview;
use leptos_router::{components::A, hooks::use_params, params::Params};
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

    let get_member = ServerAction::<api::GetMember>::new();
    let set_units = ServerAction::<api::SetUnits>::new();

    let add_unit = move |unit, member: Member| {
        set_units.dispatch(api::SetUnits {
            group: group(),
            member: member.name.clone(),
            units: member.units.tap_mut(|units| units.push(unit)),
        });
        get_member.dispatch(api::GetMember {
            group: group(),
            member: member.name.clone(),
        });
    };

    // refresh member preferences everon every group/member change
    Effect::new(move || {
        get_member.dispatch(api::GetMember {
            group: group(),
            member: member(),
        });
    });

    mview! {
        Suspense
            fallback=[mview! { p("Loading user...") }]
        (
            ErrorBoundary
                fallback={|err| mview! { "Oops!" f["{:#?}", err()] }}
            (
                [Suspend::new(async move {
                    let Some(member) = get_member.value()() else {
                        return Ok(mview! {
                            "Loading..."
                        }.into_any())
                    };
                    match member {
                        Err(_) => Err(GetError::ServerError),
                        Ok(None) => Err(GetError::MemberNotFound),
                        Ok(Some(member)) => {
                            let member2 = member.clone();
                            Ok(mview! {
                                Preferences
                                    add_unit={move |unit| add_unit(unit, member.clone())}
                                    member={member2};
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
    add_unit: impl Fn(UnitCode) + 'static,
    #[prop(into)] member: Signal<Member>,
) -> impl IntoView {
    let add_unit_input = RwSignal::new(String::new());
    mview! {
        h1({member().name})

        ul class={s::member_nav} (
            li (A href="" ("Preferences"))
            li (A href="calendar" ("Calendar"))
        )

        h2 ("Units")

        ul (
            For
                each=[member().units.clone()]
                key={String::clone}
            |unit| {
                li ({unit})
            }
        )

        input type="text" placeholder="Add unit" bind:value={add_unit_input} ()
        Button variant={ButtonVariant::Primary} on:click={move |_| {
            logging::log!("{}", add_unit_input());

            add_unit(add_unit_input());
        }} ("+")

        h2 ("Preferences")
    }
}

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("User not found.")]
    MemberNotFound,
    #[error("Server error.")]
    ServerError,
}
