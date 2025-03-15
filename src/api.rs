use leptos::prelude::*;

use backend::{activity::UnitCode, Group, Member};

#[server]
pub async fn get_group(id: String) -> Result<Option<Group>, ServerFnError> {
    Ok(backend::api::get_group(&id))
}

#[server]
pub async fn get_member(group: String, member: String) -> Result<Option<Member>, ServerFnError> {
    Ok(Some(Member {
        name: member,
        units: vec!["FIT1045".into(), "FIT1047".into()],
        preferences: vec![],
    }))
}

#[server]
pub async fn set_units(
    group: String,
    member: String,
    units: Vec<UnitCode>,
) -> Result<(), ServerFnError> {
    println!("set units {group} {member} {units:?}");
    Ok(())
}
