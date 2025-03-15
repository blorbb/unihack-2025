use std::collections::BTreeMap;

use leptos::prelude::*;

use backend::{
    activity::{Activity, Class, UnitCode},
    Animation, Group, Member,
};

#[server]
pub async fn create_group() -> Result<String, ServerFnError> {
    backend::api::create_group()
        .map_err(|_| ServerFnError::Response(String::from("TODO create group error")))
}

#[server]
pub async fn get_group(id: String) -> Result<Option<Group>, ServerFnError> {
    Ok(backend::api::get_group(&id))
}

#[server]
pub async fn add_group_member(group_id: String, member: String) -> Result<(), ServerFnError> {
    backend::api::add_group_member(&group_id, &member)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server]
pub async fn update_member(group_id: String, member: Member) -> Result<(), ServerFnError> {
    backend::api::update_member(&group_id, member)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server]
pub async fn search_units(query: String) -> Result<Vec<String>, ServerFnError> {
    Ok(backend::api::search_units(&query))
}

#[server]
pub async fn get_member_calendar(
    group_id: String,
    member: String,
) -> Result<BTreeMap<UnitCode, BTreeMap<Activity, Class>>, ServerFnError> {
    backend::api::get_member_calendar(&group_id, &member)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server]
pub async fn get_animations() -> Result<Animation, ServerFnError> {
    Ok(backend::api::get_animations().clone())
}
