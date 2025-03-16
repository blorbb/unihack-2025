use std::collections::{BTreeMap, BTreeSet};

use leptos::prelude::*;

use backend::{
    activity::{Activity, Class, UnitCode},
    members::Preference,
    Member,
};
use serde::{Deserialize, Serialize};

#[server]
pub async fn create_group() -> Result<String, ServerFnError> {
    Ok(backend::api::create_group())
}

#[server]
pub async fn get_group(id: String) -> Result<Option<GroupInfo>, ServerFnError> {
    let Some(group) = backend::api::get_group(&id) else {
        return Ok(None);
    };
    Ok(Some(GroupInfo {
        id,
        members: group
            .members
            .into_iter()
            .map(|mem| MemberInfo {
                name: mem.name,
                units: mem
                    .units
                    .into_iter()
                    .map(|unit| MemberUnitPreferences {
                        code: unit.clone(),
                        activities: backend::api::get_activities(&unit)
                            .expect("unit code should exist")
                            .into_iter()
                            .map(|activity| {
                                (
                                    activity.clone(),
                                    mem.preferences
                                        .iter()
                                        .filter_map(|pref| match pref {
                                            Preference::ShareClass(
                                                punit,
                                                pactivity,
                                                share_with,
                                            ) => (&unit == punit && &activity == pactivity)
                                                .then_some(share_with.clone()),
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                    })
                    .collect(),
            })
            .collect(),
    }))
}

#[server]
pub async fn add_group_member(group_id: String, member: String) -> Result<(), ServerFnError> {
    backend::api::add_group_member(&group_id, &member)
        .map_err(|e| ServerFnError::WrappedServerError(e))?;
    // TODO: fix issue where it doesn't refresh properly?
    leptos_axum::redirect(&format!("/g/{group_id}/{}", urlencoding::encode(&member)));
    Ok(())
}

#[server]
pub async fn update_member(group_id: String, member: MemberInfo) -> Result<(), ServerFnError> {
    backend::api::update_member(&group_id, Member::from(member))
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server]
pub async fn search_units(query: String) -> Result<Vec<String>, ServerFnError> {
    println!("searching with {query}");
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
pub async fn get_unit_activities(unit: String) -> Result<Option<Vec<String>>, ServerFnError> {
    Ok(backend::api::get_activities(&unit))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfo {
    pub id: String,
    pub members: Vec<MemberInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberInfo {
    pub name: String,
    #[serde(default)]
    pub units: Vec<MemberUnitPreferences>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberUnitPreferences {
    pub code: UnitCode,
    /// Map from activity name to people
    #[serde(default)]
    pub activities: BTreeMap<String, BTreeSet<String>>,
}

impl From<MemberInfo> for Member {
    fn from(value: MemberInfo) -> Self {
        Self {
            name: value.name,
            units: value.units.iter().map(|u| u.code.clone()).collect(),
            preferences: value
                .units
                .into_iter()
                .flat_map(|unit| {
                    unit.activities
                        .into_iter()
                        .flat_map(move |(activity, members)| {
                            let code = unit.code.clone();
                            members.into_iter().map(move |member| {
                                Preference::ShareClass(code.clone(), activity.clone(), member)
                            })
                        })
                })
                .collect(),
        }
    }
}
