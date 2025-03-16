use crate::members::Member;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Group {
    pub members: Vec<Member>,
}
impl Group {
    pub const fn new() -> Self {
        Self { members: vec![] }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}
