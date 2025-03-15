use std::{collections::BTreeMap, str::FromStr};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tap::Tap;
use uuid::Uuid;

use crate::{
    Member, TESTING,
    activity::{Activity, Class, UnitCode},
    groups::Group,
};

use super::solver::solve;

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("Invalid ID.")]
    InvalidId,
    #[error("Group not found.")]
    GroupNotFound,
    #[error("Server error.")]
    ServerError,
}

pub fn create_group() -> String {
    let id = Uuid::now_v7();
    let mut groups = state::GROUPS.lock().unwrap(); // Take lock to access inside
    groups.insert(id, GroupState::new());
    id.to_string()
}

pub fn get_group(id: &str) -> Option<Group> {
    let id = Uuid::from_str(id).ok()?;
    if TESTING {
        eprintln!("Get group request");
        dbg!(id);
        dbg!(
            state::GROUPS
                .lock()
                .unwrap()
                .get(&id)
                .map(|x| x.group.clone())
        )
    } else {
        state::GROUPS
            .lock()
            .unwrap()
            .get(&id)
            .map(|x| x.group.clone())
    }
}

// Group already exists, member is being created
pub fn add_group_member(group_id: &str, member_name: &str) -> Result<(), GetError> {
    let group_id = Uuid::from_str(group_id).map_err(|_| GetError::InvalidId)?;
    let member = Member::new(member_name);
    {
        let mut groups = state::GROUPS.lock().unwrap();
        groups
            .get_mut(&group_id)
            .ok_or(GetError::GroupNotFound)?
            .group
            .members
            .push(member);
    }

    Ok(())
}

pub fn update_member(group_id: &str, member: Member) -> anyhow::Result<()> {
    let mut groups = state::GROUPS.lock().unwrap();

    let group = groups
        .get_mut(&Uuid::from_str(group_id)?)
        .ok_or(anyhow!("Invalid group id"))?;

    group.group.members = group
        .group
        .members
        .iter()
        .filter(|x| x.name != member.name)
        .cloned()
        .collect();

    group.group.members.push(member);

    let sol = solve(&state::CLASSES, &group.group.members).0;

    group.calendar = sol;

    Ok(())
}

pub fn search_units(query: &str) -> Vec<String> {
    if query.trim().is_empty() {
        return vec![];
    }

    // TODO: fuzzy search
    // TODO: return unit name
    state::CLASSES
        .keys()
        .filter(|s| s.to_lowercase().starts_with(&query.to_lowercase()))
        .cloned()
        .collect::<Vec<_>>()
        .tap_mut(|x| x.sort())
}

pub fn get_member_calendar(
    group_id: &str,
    member: &str,
) -> anyhow::Result<BTreeMap<UnitCode, BTreeMap<Activity, Class>>> {
    let groups = state::GROUPS.lock().unwrap();

    let group_state = groups
        .get(&Uuid::from_str(group_id)?)
        .ok_or(anyhow!("Invalid group id"))?;

    Ok(group_state
        .calendar
        .get(member)
        .cloned()
        .unwrap_or_default())
}

pub fn load_classes() {
    let _ = &*state::CLASSES;
}

pub fn get_activities(unit_code: &str) -> Option<Vec<String>> {
    Some(state::CLASSES.get(unit_code)?.1.keys().cloned().collect())
}

#[derive(Debug)]
struct GroupState {
    group: Group,
    calendar: BTreeMap<String, BTreeMap<UnitCode, BTreeMap<Activity, Class>>>,
}

impl GroupState {
    fn new() -> Self {
        GroupState {
            group: Group::new(),
            calendar: BTreeMap::new(),
        }
    }
}

mod state {
    use std::{
        collections::HashMap,
        path::Path,
        sync::{LazyLock, Mutex},
    };

    use crate::{
        activity::{Classes, UnitInfo},
        classes::load_classes,
        members::Member,
    };

    use super::*;
    type MHashMap<K, V> = Mutex<HashMap<K, V>>;
    pub static GROUPS: LazyLock<MHashMap<Uuid, GroupState>> = LazyLock::new(|| {
        let mut map = HashMap::<_, _>::new();

        if TESTING {
            let mut group_state = GroupState::new();

            group_state.group.members.extend(
                vec!["bobr", "cat", "car"]
                    .into_iter()
                    .map(Member::new)
                    .collect::<Vec<_>>(),
            );

            group_state.calendar = [("bobr".to_string(),serde_json::from_str(r#"{"FIT1045":{"Applied":{"day":"Friday","code":"10_OnCampus","start":600,"end":720},"PASS-Optional":{"day":"Tuesday","code":"01_OnCampus","start":960,"end":1020},"Workshop-JTA":{"day":"Thursday","code":"03_OnCampus","start":480,"end":600}},"FIT1047":{"Applied":{"day":"Thursday","code":"17_OnCampus","start":960,"end":1080},"PASS-Optional":{"day":"Wednesday","code":"01_OnCampus","start":900,"end":960},"Workshop":{"day":"Friday","code":"01_OnCampus","start":720,"end":840}},"MAT1830":{"Applied":{"day":"Friday","code":"11_OnCampus","start":840,"end":960},"Seminar_1":{"day":"Tuesday","code":"02_OnCampus","start":780,"end":840},"Seminar_2":{"day":"Thursday","code":"01_OnlineRealTIme","start":840,"end":900},"Seminar_3":{"day":"Friday","code":"01_OnCampus","start":960,"end":1020}},"MTH1030":{"Applied":{"day":"Friday","code":"01_OnCampus","start":480,"end":600},"Seminar_1-JTA":{"day":"Thursday","code":"01_OnCampus","start":600,"end":720},"Seminar_2-JTA":{"day":"Thursday","code":"01_OnCampus","start":780,"end":840}}}"#).unwrap())].into();

            map.insert(Uuid::nil(), group_state);
        }

        Mutex::new(map)
    });
    pub static CLASSES: LazyLock<HashMap<UnitCode, (UnitInfo, Classes)>> = LazyLock::new(|| {
        load_classes(Path::new("./class-data/classes")).expect("Missing class-data")
    });
}
