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

    mview! {
        h1 class={s::header} ({member.read().name.clone()})

        h2 ("Units")

        ul (
            For
                each=[member.read().units.clone()]
                key={String::clone}
            |unit| {
                li ({unit})
            }
        )

        input type="text" placeholder="Add unit";
        Button variant={ButtonVariant::Primary} ("+")

        h2 ("Preferences")
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
