use serde::{Deserialize, Serialize};
pub use uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub members: Vec<Member>,
}

pub type Member = String;
