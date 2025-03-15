use std::collections::HashMap;

use num_derive::FromPrimitive;

pub type UnitCode = String;
pub type Activity = String;

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
