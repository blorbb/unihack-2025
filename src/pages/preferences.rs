use backend::{Member, UnitCode, UserInfo};
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
    member: String,
}

#[component]
pub fn PreferencesPage() -> impl IntoView {
    let param = use_params::<PreferencesParams>();
    let member = move || {
        param
            .read()
            .as_ref()
            .map(|params| params.member.clone())
            .unwrap_or_default()
    };

    let get_info = ServerAction::<api::GetMember>::new();
    let set_units = ServerAction::<api::SetUnits>::new();

    let add_unit = move |unit, info: UserInfo| {
        set_units.dispatch(api::SetUnits {
            group: "00000000-0000-0000-0000-000000000000".to_owned(),
            member: member(),
            units: info.units.tap_mut(|units| units.push(unit)),
        });
        get_info.dispatch(api::GetMember {
            group: "00000000-0000-0000-0000-000000000000".to_owned(),
            member: member(),
        });
    };

    get_info.dispatch(api::GetMember {
        group: "00000000-0000-0000-0000-000000000000".to_owned(),
        member: member(),
    });

    mview! {
        Suspense
            fallback=[mview! { p("Loading user...") }]
        (
            ErrorBoundary
                fallback={|err| mview! { "Oops!" f["{:#?}", err()] }}
            (
                [Suspend::new(async move {
                    let Some(info) = get_info.value()() else {
                        return Ok(mview! {
                            "Loading..."
                        }.into_any())
                    };
                    match info {
                        Err(_) => Err(GetError::ServerError),
                        Ok(None) => Err(GetError::MemberNotFound),
                        Ok(Some(info)) => {
                            let info2 = info.clone();
                            Ok(mview! {Preferences add_unit={move |unit| add_unit(unit, info.clone())} member={member()} info={info2}; }.into_any())
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
    #[prop(into)] member: Member,
    #[prop(into)] info: Signal<UserInfo>,
) -> impl IntoView {
    let add_unit_input = RwSignal::new(String::new());
    mview! {
        nav (
            ul class={s::member_nav} (
                li (A href="" ("Preferences"))
                li (A href="calendar" ("Calendar"))
            )
        )

        h2 ("Units")

        ul (
            For
                each=[info().units.clone()]
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
