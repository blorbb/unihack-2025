use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::{components::A, hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

stylance::import_style!(s, "group.module.scss");

#[derive(Params, PartialEq)]
struct GroupParams {
    group: String,
}

#[component]
pub fn GroupPage() -> impl IntoView {
    let param = use_params::<GroupParams>();
    let group = move || {
        param
            .read()
            .as_ref()
            .ok()
            .map(|params| params.group.clone())
            .unwrap_or_default()
    };
    let group_resource = Resource::new(group, get_group);

    mview! {
        Suspense
            fallback=[mview! { p("Loading group...") }]
        (
            ErrorBoundary
                fallback={|err| mview! { "Oops!" f["{:#?}", err()] }}
            (
                [Suspend::new(async move {
                    let group = group_resource.await;
                    let view = match group {
                        Ok(Some(g)) => mview! {
                            GroupList group={g};
                        },
                        Ok(None) => return Err(GetError::GroupNotFound),
                        Err(e) => return Err(GetError::ServerError)
                    };
                    Ok(view)
                })]
            )
        )
    }
}

#[component]
fn GroupList(#[prop(into)] group: Signal<backend::Group>) -> impl IntoView {
    mview! {
        div class={s::member_list_wrapper} (
            h2("Group Members")
            ul class={s::member_list} (
                For
                    each=[group.read().members.clone()]
                    key={|member| member.clone()}
                |member| {
                    li class={s::member} (
                        A attr:class={s::member_link} href={urlencoding::encode(&member).into_owned()} (
                            span class={s::member_name} ({member})
                            span class={s::member_units} ("TODO")
                        )
                    )
                }
            )
        )
    }
}

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("Invalid group ID.")]
    InvalidId,
    #[error("Group not found.")]
    GroupNotFound,
    #[error("Server error.")]
    ServerError,
}

#[server]
pub async fn get_group(id: String) -> Result<Option<backend::Group>, ServerFnError> {
    Ok(backend::server::groups::get_group(&id))
}
