use anyhow::Result;
use backend::server::solver::{ClassTimes, solve};
use backend::{UserInfo, Username};
use std::collections::HashMap;
use std::path::Path;

type Constraints = HashMap<Username, UserInfo>;

#[tokio::main]
async fn main() -> Result<()> {
    let classes: ClassTimes =
        backend::server::classes::load_classes(Path::new("../class-data/classes"))
            .await?
            .into_iter()
            .map(|(a, (b, c))| (a, c))
            .collect();

    let constraints: Constraints = [(
        "A".to_string(),
        UserInfo {
            units: vec!["FIT1045".to_string()],
            preferences: vec![],
        },
    )]
    .into();

    let solution = solve(&classes, &constraints);

    println!("{:?}", solution);

    Ok(())
}
