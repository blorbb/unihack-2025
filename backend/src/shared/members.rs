use serde::{Deserialize, Serialize};

type Preference = ();
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub preferences: Vec<Preference>,
}

impl Member {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            preferences: vec![],
        }
    }
}
