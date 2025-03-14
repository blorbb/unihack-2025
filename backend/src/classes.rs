use std::collections::HashMap;

pub type UnitCode = String;
pub type Activity = String;
pub type Username = String;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class {
    pub day: WeekDay,

    // Minutes from midnight
    pub start: usize,
    pub end: usize,
}

pub type ClassTimes = HashMap<UnitCode, HashMap<Activity, Vec<Class>>>;

pub struct Units {
    pub class_times: ClassTimes,
}

pub struct UnitInfo {
    pub name: String,
}

pub enum Preference {
    ShareClass(UnitCode, Activity, Username),
}

pub struct UserInfo {
    pub units: Vec<UnitCode>,
    pub preferences: Vec<Preference>,
}
