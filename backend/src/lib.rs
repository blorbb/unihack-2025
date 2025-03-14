#![feature(try_blocks)]
#![feature(let_chains)]

use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

type UnitCode = String;
type Activity = String;
type Username = String;

#[derive(Clone, Copy, Hash)]
enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(Clone, Copy, Hash)]
struct Class {
    day: WeekDay,

    // Minutes from midnight
    start: usize,
    end: usize,
}

type ClassTimes = HashMap<UnitCode, HashMap<Activity, Vec<Class>>>;

struct Units {
    class_times: ClassTimes,
}

struct UnitInfo {
    name: String,
}

enum Preference {
    ShareClass(UnitCode, Activity, Username),
}

struct UserInfo {
    units: Vec<UnitCode>,
    preferences: Vec<Preference>,
}

pub mod solver {
    use std::collections::BTreeMap;

    use super::*;

    type Solution = BTreeMap<Username, BTreeMap<UnitCode, BTreeMap<Activity, Class>>>;

    pub fn solve(class_times: &ClassTimes, users: &HashMap<Username, UserInfo>) -> Solution {
        Solution::new()
    }
}

pub fn increment(message: &str) -> u32 {
    static N: AtomicU32 = AtomicU32::new(0);
    N.fetch_add(1, Ordering::Relaxed)
}
