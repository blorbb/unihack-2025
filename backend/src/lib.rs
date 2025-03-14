#![feature(try_blocks)]
#![feature(let_chains)]
use std::sync::atomic::{AtomicU32, Ordering};
use std::{
    collections::{HashMap, hash_map::Entry},
    sync::{LazyLock, Mutex},
};

const TESTING: bool = true;

use serde::{Deserialize, Serialize};
type Member = String;
type GroupName = String;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub name: GroupName,
    pub members: Vec<Member>,
}
impl Group {
    const fn new(name: GroupName) -> Self {
        Self {
            name,
            members: vec![],
        }
    }
}
type MHashMap<K, V> = Mutex<HashMap<K, V>>;
static GROUPS: LazyLock<MHashMap<GroupName, Group>> = LazyLock::new(|| {
    let mut map = HashMap::<_, _>::new();
    if TESTING {
        let mut group = Group::new(String::new());
        group.members.extend(
            vec!["bobr", "cat", "car"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>(),
        );
        map.insert(String::new(), group);
    }
    Mutex::new(map)
});

pub mod classes;

// TODO(delete):
pub fn increment(message: &str) -> u32 {
    static N: AtomicU32 = AtomicU32::new(0);

    println!("got a message on the server: {message}");
    N.fetch_add(1, Ordering::Relaxed)
}

// TODO: Give a real Error
pub fn create_group(name: &str) -> Result<String, ()> {
    let mut groups = GROUPS.lock().unwrap(); // Take lock to access inside
    match groups.entry(name.to_string()) {
        Entry::Occupied(_) => return Err(()),
        Entry::Vacant(x) => x.insert_entry(Group::new(name.to_string())),
    };
    Ok(name.to_string())
}

pub fn get_group(name: &str) -> Option<Group> {
    if TESTING {
        eprintln!("Get group request");
        dbg!(name);
        dbg!(GROUPS.lock().unwrap().get(name).cloned())
    } else {
        GROUPS.lock().unwrap().get(name).cloned()
    }
}
