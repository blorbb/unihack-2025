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

use crate::{Member, members::Preference, shared::activity::*};

const POPULATION: usize = 200;
const ITERATIONS: usize = 200;
const MUTATIONS: usize = 5;

type Solution = BTreeMap<String, BTreeMap<UnitCode, BTreeMap<Activity, Class>>>;

pub type ClassTimes = HashMap<UnitCode, (UnitInfo, Classes)>;

fn score_solve(members: &Vec<Member>, solution: &Solution) -> i64 {
    let mut ans: i64 = 0;

    // Check no overlapping //////////
    for (_, units) in solution {
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

                if a.day == b.day && max(a.start, b.start) < min(a.end, b.end) {
                    ans -= 10
                }
            }
        }
    }

    for member in members {
        for preference in member.preferences.iter() {
            match preference {
                Preference::ShareClass(unitcode, activity, member_b) => {
                    if let (Some(class_a), Some(class_b)) = (
                        solution[&member.name]
                            .get(unitcode)
                            .and_then(|x| x.get(activity)),
                        solution[member_b]
                            .get(unitcode)
                            .and_then(|x| x.get(activity)),
                    ) {
                        if class_a == class_b {
                            ans += 1
                        }
                    }
                }
            }
        }
    }

    ans
}

fn random_sol(class_times: &ClassTimes, users: &Vec<Member>, rng: &mut ThreadRng) -> Solution {
    users
        .iter()
        .map(|member| {
            (
                member.name.clone(),
                member
                    .units
                    .iter()
                    .map(|unit_code| {
                        (
                            unit_code.clone(),
                            class_times[unit_code]
                                .1
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
        let (_, units) = solution.iter_mut().choose(rng).unwrap();
        let (unit, activities) = units.iter_mut().choose(rng).unwrap();
        let (activity, class) = activities.iter_mut().choose(rng).unwrap();

        *class = class_times[unit].1[activity].choose(rng).unwrap().clone();
    }

    solution
}

pub fn solve(class_times: &ClassTimes, members: &Vec<Member>) -> (Solution, i64) {
    let mut rng = rng();

    let mut solutions: BTreeMap<(i64, u64), Solution> = BTreeMap::new();

    while solutions.len() < POPULATION {
        let solution = random_sol(class_times, members, &mut rng);
        let score = score_solve(members, &solution);
        let hash = hash_solve(&solution);
        solutions.insert((score, hash), solution);
    }

    for iteration in 0..ITERATIONS {
        while solutions.len() > POPULATION / 2 {
            solutions.first_entry().unwrap().remove();
        }

        while (solutions.len() < POPULATION) {
            let solution = new_sol(
                class_times,
                solutions.iter().choose(&mut rng).unwrap().1,
                &mut rng,
            );
            let score = score_solve(members, &solution);
            let hash = hash_solve(&solution);
            solutions.insert((score, hash), solution);
        }

        #[cfg(debug_assertions)]
        println!(
            "Iteration: {}, Scores: {:?}",
            iteration,
            solutions
                .iter()
                .map(|((a, _), _)| { a })
                .collect::<Vec<_>>()
        );
    }

    let best = solutions.pop_last().unwrap();
    (best.1, best.0.0)
}
