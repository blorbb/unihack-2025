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

const POPULATION: usize = 300;
const MAX_ITERATIONS: usize = 500;
const ITERATIONS_WITHOUT_IMPROVEMENT: usize = 100;
const MUTATIONS_PER_PERSON: usize = 2;

type Solution = BTreeMap<String, BTreeMap<UnitCode, BTreeMap<Activity, Class>>>;

pub type ClassTimes = HashMap<UnitCode, (UnitInfo, Classes)>;

fn score_solve(members: &Vec<Member>, solution: &Solution) -> i64 {
    let mut ans: i64 = 0;

    // Check no overlapping //////////
    for units in solution.values() {
        let mut user_classes: Vec<Class> = Vec::new();

        for activities in units.values() {
            for class in activities.values() {
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
                Preference::ShareClass(unit_code, activity, member_b) => {
                    if let (Some(class_a), Some(class_b)) = (
                        solution[&member.name]
                            .get(unit_code)
                            .and_then(|x| x.get(activity)),
                        solution[member_b]
                            .get(unit_code)
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

fn random_sol(class_times: &ClassTimes, users: &[Member], rng: &mut ThreadRng) -> Solution {
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

fn new_sol(
    class_times: &ClassTimes,
    solution: &Solution,
    rng: &mut ThreadRng,
    people: usize,
    classes_to_change: &Vec<(String, UnitCode, Activity)>,
) -> Solution {
    let mut solution = solution.clone();

    for _ in 0..(MUTATIONS_PER_PERSON * people) {
        let (username, unit_code, activity) = classes_to_change.choose(rng).unwrap();
        let new_class = class_times[unit_code].1[activity]
            .choose(rng)
            .unwrap()
            .clone();

        solution
            .get_mut(username)
            .unwrap()
            .get_mut(unit_code)
            .unwrap()
            .insert(activity.clone(), new_class);
    }

    solution
}

fn classes_to_change(
    class_times: &ClassTimes,
    members: &Vec<Member>,
) -> Vec<(String, UnitCode, Activity)> {
    let mut classes_to_change: Vec<(String, UnitCode, Activity)> = Vec::new();

    members.iter().for_each(|member| {
        member.units.iter().for_each(|unit| {
            class_times[unit].1.iter().for_each(|(activity, _)| {
                classes_to_change.push((member.name.clone(), unit.clone(), activity.clone()))
            })
        })
    });

    classes_to_change
}

pub fn solve(class_times: &ClassTimes, members: &Vec<Member>) -> (Solution, i64) {
    if members.is_empty() {
        return (Solution::new(), 0);
    }

    let mut rng = rng();

    let mut solutions: BTreeMap<(i64, u64), Solution> = BTreeMap::new();

    let classes_to_change = classes_to_change(class_times, members);

    if classes_to_change.len() == 0 {
        return (random_sol(class_times, &members, &mut rng), -100);
    }

    while solutions.len() < POPULATION {
        let solution = random_sol(class_times, members, &mut rng);
        let score = score_solve(members, &solution);
        let hash = hash_solve(&solution);
        solutions.insert((score, hash), solution);
    }

    let mut best_score = solutions.last_key_value().unwrap().0.0;
    let mut iterations_without_improvement = 0;

    for iteration in 0..MAX_ITERATIONS {
        while solutions.len() > POPULATION / 2 {
            solutions.first_entry().unwrap().remove();
        }

        while solutions.len() < POPULATION {
            let solution = new_sol(
                class_times,
                solutions.iter().choose(&mut rng).unwrap().1,
                &mut rng,
                members.len(),
                &classes_to_change,
            );
            let score = score_solve(members, &solution);
            let hash = hash_solve(&solution);
            solutions.insert((score, hash), solution);
        }

        if solutions.last_key_value().unwrap().0.0 == best_score {
            iterations_without_improvement += 1;
            if (iterations_without_improvement > ITERATIONS_WITHOUT_IMPROVEMENT) {
                break;
            };
        } else {
            iterations_without_improvement = 0;
            best_score = solutions.last_key_value().unwrap().0.0;
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
