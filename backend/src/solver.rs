use std::{
    collections::{BTreeMap, HashMap},
    hash::{DefaultHasher, Hash, Hasher},
};

use rand::{
    rng,
    rngs::ThreadRng,
    seq::{IndexedRandom, IteratorRandom},
};

use super::classes::*;

const POPULATION: usize = 50;
const ITERATIONS: usize = 100;
const MUTATIONS: usize = 3;

type Solution = BTreeMap<Username, BTreeMap<UnitCode, BTreeMap<Activity, Class>>>;

fn score_solve(_users: &HashMap<Username, UserInfo>, _solution: &Solution) -> i64 {
    0
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
                                    (activity.clone(), *classes.choose(rng).unwrap())
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

pub fn hash_solve(solution: &Solution) -> u64 {
    let mut hasher = DefaultHasher::new();
    solution.hash(&mut hasher);
    hasher.finish()
}

pub fn new_sol(class_times: &ClassTimes, solution: &Solution, rng: &mut ThreadRng) -> Solution {
    let mut solution = solution.clone();

    for _ in 0..MUTATIONS {
        let (username, units) = solution.iter_mut().choose(rng).unwrap();
        let (unit, activities) = units.iter_mut().choose(rng).unwrap();
        let (_, class) = activities.iter_mut().choose(rng).unwrap();

        *class = *class_times[username][unit].choose(rng).unwrap();
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
        while (solutions.len() > POPULATION / 2) {
            solutions.first_entry().unwrap().remove();
        }
    }
}
