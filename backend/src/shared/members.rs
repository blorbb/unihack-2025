use serde::{Deserialize, Serialize};

use super::activity::{Activity, UnitCode};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Preference {
    ShareClass(UnitCode, Activity, String),
}

#[derive(Debug, Clone)]
pub struct Member {
    pub name: String,
    pub preferences: Vec<Preference>,
    pub units: Vec<UnitCode>,
}

impl Member {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            preferences: vec![],
            units: vec![],
        }
    }
}
