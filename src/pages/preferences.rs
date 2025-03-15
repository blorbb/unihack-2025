use backend::UserInfo;
use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::{components::A, hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

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
    let member_resource = Resource::new(member, |member| {
        get_member("00000000-0000-0000-0000-000000000000".to_owned(), member)
    });

    mview! {
        Suspense
            fallback=[mview! { p("Loading user...") }]
        (
            ErrorBoundary
                fallback={|err| mview! { "Oops!" f["{:#?}", err()] }}
            (
                [Suspend::new(async move {
                    let member = member_resource.await;
                    Ok(match member {
                        Ok(Some(m)) => mview! { Preferences member={m}; },
                        Ok(None) => return Err(GetError::MemberNotFound),
                        Err(_) => return Err(GetError::ServerError)
                    })
                })]
            )
        )
    }
}

#[component]
pub fn Preferences(#[prop(into)] member: Signal<UserInfo>) -> impl IntoView {
    mview! {
        nav (
            ul class={s::member_nav} (
                li (A href="" ("Preferences"))
                li (A href="calendar" ("Calendar"))
            )
        )
    }
}

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("User not found.")]
    MemberNotFound,
    #[error("Server error.")]
    ServerError,
}

#[server]
pub async fn get_member(group: String, member: String) -> Result<Option<UserInfo>, ServerFnError> {
    Ok(Some(UserInfo {
        units: vec![],
        preferences: vec![],
    }))
}
