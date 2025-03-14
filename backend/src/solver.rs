use super::classes::{Activity, Class, ClassTimes, UnitCode, UserInfo, Username};
use std::collections::{BTreeMap, HashMap};

type Solution = BTreeMap<Username, BTreeMap<UnitCode, BTreeMap<Activity, Class>>>;

pub fn solve(class_times: &ClassTimes, users: &HashMap<Username, UserInfo>) -> Solution {
    Solution::new()
}
