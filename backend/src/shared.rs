use std::collections::HashMap;

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

pub use uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub members: Vec<Member>,
}

pub type Member = String;

pub type UnitCode = String;
pub type Activity = String;
pub type Username = String;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class {
    pub day: WeekDay,
    pub code: String,

    /// Minutes from midnight
    pub start: u16,
    /// Minutes from midnight
    pub end: u16,
}

pub type Classes = HashMap<Activity, Vec<Class>>;

#[derive(Clone, Debug)]
pub struct UnitInfo {
    pub name: String,
}

#[derive(Clone, Debug)]
pub enum Preference {
    ShareClass(UnitCode, Activity, Username),
}

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub units: Vec<UnitCode>,
    pub preferences: Vec<Preference>,
}
