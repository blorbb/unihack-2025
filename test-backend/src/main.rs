use anyhow::Result;
use backend::{
    Member,
    solver::{ClassTimes, solve},
};
use std::{collections::HashMap, path::Path};

#[tokio::main]
async fn main() -> Result<()> {
    let classes: ClassTimes = backend::classes::load_classes(Path::new("../class-data/classes"))
        .await?
        .into_iter()
        .map(|(a, (b, c))| (a, c))
        .collect();

    let members = vec![
        (Member {
            name: "a".to_string(),
            units: vec!["FIT1045".to_string()],
            preferences: vec![],
        }),
    ];

    let solution = solve(&classes, &members);

    println!("{:?}", solution);

    Ok(())
}
