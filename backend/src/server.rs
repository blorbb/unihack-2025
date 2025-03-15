const TESTING: bool = true;

pub mod classes;
pub mod solver;

pub mod groups {
    use crate::Group;

    use super::TESTING;
    use std::{
        collections::{HashMap, hash_map::Entry},
        str::FromStr,
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

    pub fn get_group(id: &str) -> Option<Group> {
        let id = Uuid::from_str(id).ok()?;
        if TESTING {
            eprintln!("Get group request");
            dbg!(id);
            dbg!(GROUPS.lock().unwrap().get(&id).cloned())
        } else {
            GROUPS.lock().unwrap().get(&id).cloned()
        }
    }
}
