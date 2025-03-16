use leptos::prelude::*;
use leptos_mview::mview;
use leptos_router::{
    components::{Outlet, A},
    hooks::use_params,
    params::Params,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::{self, AddGroupMember, GroupInfo},
    components::{button::ButtonVariant, Button},
};

stylance::import_style!(s, "group_layout.module.scss");

#[derive(Params, PartialEq)]
struct GroupParams {
    group: String,
}

#[component]
pub fn GroupLayout() -> impl IntoView {
    let param = use_params::<GroupParams>();
    let group = move || {
        param
            .read()
            .as_ref()
            .ok()
            .map(|params| params.group.clone())
            .unwrap_or_default()
    };
    let group_resource = Resource::new(group, api::get_group);

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
                            div class={s::layout} (
                                GroupList group={g};
                                main(Outlet;)
                            )
                        },
                        Ok(None) => return Err(GetError::GroupNotFound),
                        Err(_) => return Err(GetError::ServerError)
                    };
                    Ok(view)
                })]
            )
        )
    }
}

#[component]
fn GroupList(group: GroupInfo) -> impl IntoView {
    let group = StoredValue::new(group);
    let add_group_member = ServerAction::<AddGroupMember>::new();

    mview! {
        nav class={s::member_list_wrapper} (
            h1 class={s::home_link} (A href="/" ("una 📅"))

            h2("Group Members")
            ul (
                For
                    each=[group.read_value().members.iter().map(|m| m.name.clone()).collect::<Vec<_>>()]
                    key={|member| member.clone()}
                |member| {
                    // TODO: delete member button
                    li class={s::member} (
                        A attr:class={s::member_link} href={urlencoding::encode(&member).into_owned()} (
                            span class={s::member_name} ({member})
                        )
                    )
                }

                li class={s::member} (
                    ActionForm attr:class={s::add_member_form} action={add_group_member} (
                        input type="hidden" name="group_id" prop:value={group.read_value().id.clone()};
                        input class={s::member_link}
                            name="member" placeholder="Add member";
                        Button
                            class={s::add_member_button}
                            variant={ButtonVariant::Primary}
                            type="submit"
                        (
                            "+"
                        )
                    )
                )
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
