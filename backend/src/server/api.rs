use std::{
    collections::{BTreeMap, hash_map::Entry},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    Member, TESTING,
    activity::{Activity, Class, UnitCode},
    groups::Group,
    members,
};

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("Invalid ID.")]
    InvalidId,
    #[error("Group not found.")]
    GroupNotFound,
    #[error("Server error.")]
    ServerError,
}
// TODO: Give a real Error
pub fn create_group() -> Result<String, ()> {
    let id = Uuid::now_v7();
    let mut groups = state::GROUPS.lock().unwrap(); // Take lock to access inside
    match groups.entry(id) {
        Entry::Occupied(_) => return Err(()),
        Entry::Vacant(x) => x.insert_entry(Group::new()),
    };
    Ok(id.to_string())
}

pub fn get_group(id: &str) -> Option<Group> {
    let id = Uuid::from_str(id).ok()?;
    if TESTING {
        eprintln!("Get group request");
        dbg!(id);
        dbg!(state::GROUPS.lock().unwrap().get(&id).cloned())
    } else {
        state::GROUPS.lock().unwrap().get(&id).cloned()
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
            .members
            .push(member);
    }

    Ok(())
}

pub fn get_member_preferences() {
    todo!()
}

pub fn get_member_calendar(
    _group_id: &str,
    _member_name: &str,
) -> BTreeMap<UnitCode, BTreeMap<Activity, Class>> {
    serde_json::from_str(r#"{"FIT1045":{"Applied":{"day":"Friday","code":"10_OnCampus","start":600,"end":720},"PASS-Optional":{"day":"Tuesday","code":"01_OnCampus","start":960,"end":1020},"Workshop-JTA":{"day":"Thursday","code":"03_OnCampus","start":480,"end":600}},"FIT1047":{"Applied":{"day":"Thursday","code":"17_OnCampus","start":960,"end":1080},"PASS-Optional":{"day":"Wednesday","code":"01_OnCampus","start":900,"end":960},"Workshop":{"day":"Friday","code":"01_OnCampus","start":720,"end":840}},"MAT1830":{"Applied":{"day":"Friday","code":"11_OnCampus","start":840,"end":960},"Seminar_1":{"day":"Tuesday","code":"02_OnCampus","start":780,"end":840},"Seminar_2":{"day":"Thursday","code":"01_OnlineRealTIme","start":840,"end":900},"Seminar_3":{"day":"Friday","code":"01_OnCampus","start":960,"end":1020}},"MTH1030":{"Applied":{"day":"Friday","code":"01_OnCampus","start":480,"end":600},"Seminar_1-JTA":{"day":"Thursday","code":"01_OnCampus","start":600,"end":720},"Seminar_2-JTA":{"day":"Thursday","code":"01_OnCampus","start":780,"end":840}}}"#).unwrap()
}

mod state {
    use std::{
        collections::HashMap,
        sync::{LazyLock, Mutex},
    };

    use crate::members::Member;

    use super::*;
    type MHashMap<K, V> = Mutex<HashMap<K, V>>;
    pub static GROUPS: LazyLock<MHashMap<Uuid, Group>> = LazyLock::new(|| {
        let mut map = HashMap::<_, _>::new();
        if TESTING {
            let mut group = Group::new();
            group.members.extend(
                vec!["bobr", "cat", "car"]
                    .into_iter()
                    .map(Member::new)
                    .collect::<Vec<_>>(),
            );
            map.insert(Uuid::nil(), group);
        }
        Mutex::new(map)
    });
    pub static MEMBERS: LazyLock<MHashMap<String, Member>> = LazyLock::new(|| {
        let mut map = HashMap::<_, _>::new();
        if TESTING {
            let member = Member::new("Testing");
            map.insert("Testing".to_owned(), member);
        }
        Mutex::new(map)
    });
}
