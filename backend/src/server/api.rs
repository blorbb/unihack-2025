use std::{collections::hash_map::Entry, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{Member, TESTING, groups::Group, members};

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

pub fn get_member_calendar() {
    todo!()
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
