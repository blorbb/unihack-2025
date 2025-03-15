#![feature(try_blocks)]
#![feature(let_chains)]

const TESTING: bool = true;

pub mod classes;

pub mod groups {
    use super::TESTING;
    use serde::{Deserialize, Serialize};
    use std::{
        collections::{HashMap, hash_map::Entry},
        sync::{LazyLock, Mutex},
    };
    use uuid::Uuid;
    type MHashMap<K, V> = Mutex<HashMap<K, V>>;
    static GROUPS: LazyLock<MHashMap<Uuid, Group>> = LazyLock::new(|| {
        let mut map = HashMap::<_, _>::new();
        if TESTING {
            let mut group = Group::new();
            group.members.extend(
                vec!["bobr", "cat", "car"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>(),
            );
            map.insert(Uuid::nil(), group);
        }
        Mutex::new(map)
    });
    type Member = String;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Group {
        pub members: Vec<Member>,
    }
    impl Group {
        const fn new() -> Self {
            Self { members: vec![] }
        }
    }
    // TODO: Give a real Error
    pub fn create_group() -> Result<String, ()> {
        let id = Uuid::now_v7();
        let mut groups = GROUPS.lock().unwrap(); // Take lock to access inside
        match groups.entry(id) {
            Entry::Occupied(_) => return Err(()),
            Entry::Vacant(x) => x.insert_entry(Group::new()),
        };
        Ok(id.to_string())
    }

    pub fn get_group(id: Uuid) -> Option<Group> {
        if TESTING {
            eprintln!("Get group request");
            dbg!(id);
            dbg!(GROUPS.lock().unwrap().get(&id).cloned())
        } else {
            GROUPS.lock().unwrap().get(&id).cloned()
        }
    }
}
