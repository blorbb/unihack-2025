use std::{
    cmp::{max, min},
    collections::{BTreeMap, HashMap},
    hash::{DefaultHasher, Hash, Hasher},
};

use rand::{
    rng,
    rngs::ThreadRng,
    seq::{IndexedRandom, IteratorRandom},
};

use crate::shared::activity::*;

const POPULATION: usize = 50;
const ITERATIONS: usize = 100;
const MUTATIONS: usize = 3;

type Solution = BTreeMap<Username, BTreeMap<UnitCode, BTreeMap<Activity, Class>>>;

pub type ClassTimes = HashMap<UnitCode, Classes>;

fn score_solve(users: &HashMap<Username, UserInfo>, solution: &Solution) -> i64 {
    let mut ans: i64 = 0;

    // Check no overlapping //////////
    for (user, units) in solution {
        let mut user_classes: Vec<Class> = Vec::new();

        for (_, activities) in units {
            for (_, class) in activities {
                user_classes.push(class.clone());
            }
        }

        for i in 0..user_classes.len() {
            for j in (i + 1)..user_classes.len() {
                let a = &user_classes[i];
                let b = &user_classes[j];

                if a.day == b.day && max(a.start, b.start) > min(a.end, b.end) {
                    ans -= 10
                }
            }
        }
    }

    for (username, user_info) in users {
        for preference in user_info.preferences.iter() {
            match preference {
                Preference::ShareClass(unitcode, activity, username_b) => {
                    let class_a = solution[username]
                        .get(unitcode)
                        .and_then(|x| x.get(activity));
                    let class_b = solution[username_b]
                        .get(unitcode)
                        .and_then(|x| x.get(activity));
                    if class_a == class_b {
                        ans += 1
                    }
                }
            }
        }
    }

    ans
}

fn random_sol(
    class_times: &ClassTimes,
    users: &HashMap<Username, UserInfo>,
    rng: &mut ThreadRng,
) -> Solution {
    users
        .iter()
        .map(|(username, user_info)| {
            (
                username.clone(),
                user_info
                    .units
                    .iter()
                    .map(|unit_code| {
                        (
                            unit_code.clone(),
                            class_times[unit_code]
                                .iter()
                                .map(|(activity, classes)| {
                                    (activity.clone(), classes.choose(rng).unwrap().clone())
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn hash_solve(solution: &Solution) -> u64 {
    let mut hasher = DefaultHasher::new();
    solution.hash(&mut hasher);
    hasher.finish()
}

fn new_sol(class_times: &ClassTimes, solution: &Solution, rng: &mut ThreadRng) -> Solution {
    let mut solution = solution.clone();

    for _ in 0..MUTATIONS {
        let (username, units) = solution.iter_mut().choose(rng).unwrap();
        let (unit, activities) = units.iter_mut().choose(rng).unwrap();
        let (_, class) = activities.iter_mut().choose(rng).unwrap();

        *class = class_times[username][unit].choose(rng).unwrap().clone();
    }

    solution
}

pub fn solve(class_times: &ClassTimes, users: &HashMap<Username, UserInfo>) {
    let mut rng = rng();

    let mut solutions: BTreeMap<(i64, u64), Solution> = BTreeMap::new();

    while solutions.len() < POPULATION {
        let solution = random_sol(class_times, users, &mut rng);
        let score = score_solve(users, &solution);
        let hash = hash_solve(&solution);
        solutions.insert((score, hash), solution);
    }

    for _ in 0..ITERATIONS {
        while solutions.len() > POPULATION / 2 {
            solutions.first_entry().unwrap().remove();
        }
        let solution = new_sol(
            class_times,
            solutions.iter().choose(&mut rng).unwrap().1,
            &mut rng,
        );
        let score = score_solve(users, &solution);
        let hash = hash_solve(&solution);
        solutions.insert((score, hash), solution);
    }
}
