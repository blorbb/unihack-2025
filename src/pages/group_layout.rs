use leptos::{ev, html, prelude::*, task::spawn_local};
use leptos_mview::mview;
use leptos_router::{
    components::{Outlet, A},
    hooks::use_params,
    params::Params,
};
use leptos_use::{use_clipboard, UseClipboardReturn};
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
                                GroupList group={g} on_add=[group_resource.refetch()];
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
fn GroupList(group: GroupInfo, on_add: impl Fn() + Clone + Send + Sync + 'static) -> impl IntoView {
    let group = StoredValue::new(group);
    let member_input = NodeRef::<html::Input>::new();
    let add_group_member = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        match AddGroupMember::from_event(&ev) {
            Ok(new_input) => {
                let on_add = on_add.clone();
                spawn_local(async move {
                    api::add_group_member(new_input.group_id, new_input.member)
                        .await
                        .unwrap();
                    on_add();
                    let el = member_input.get().unwrap();
                    el.set_value("");
                })
            }
            Err(err) => {
                leptos::logging::error!(
                    "Error converting form field into server function \
                     arguments: {err:?}"
                );
            }
        }
    };
    let UseClipboardReturn { copy, .. } = use_clipboard();

    mview! {
        nav class={s::member_list_wrapper} (
            h1 class={s::home_link} (A href="/" ("una 📅"))

            Button
                variant={ButtonVariant::Secondary}
                class={s::copy_id_button}
                on:click={move |_| copy(&group.read_value().id) }
                ("Copy Group ID")

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
                    form method="POST" class={s::add_member_form} on:submit={add_group_member} (
                        input type="hidden" name="group_id" prop:value={group.read_value().id.clone()};
                        input class={s::member_link} ref={member_input}
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
