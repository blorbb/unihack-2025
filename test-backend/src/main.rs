use anyhow::Result;
use backend::{
    Member,
    members::Preference,
    solver::{ClassTimes, solve},
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let classes: ClassTimes = backend::classes::load_classes(Path::new("../class-data/classes"))
        .await?
        .into_iter()
        .map(|(a, (_, c))| (a, c))
        .collect();

    let members = vec![
        (Member {
            name: "a".to_string(),
            units: vec![
                "FIT1045".to_string(),
                "FIT1047".to_string(),
                "MTH1030".to_string(),
                "MAT1830".to_string(),
            ],
            preferences: vec![Preference::ShareClass(
                "FIT1045".to_string(),
                "Applied".to_string(),
                "b".to_string(),
            )],
        }),
        (Member {
            name: "b".to_string(),
            units: vec![
                "FIT1045".to_string(),
                "FIT1049".to_string(),
                "MTH1020".to_string(),
                "FIT1051".to_string(),
            ],
            preferences: vec![],
        }),
        (Member {
            name: "c".to_string(),
            units: vec![
                "FIT2004".to_string(),
                "FIT2099".to_string(),
                "FIT1049".to_string(),
                "FIT1047".to_string(),
            ],
            preferences: vec![
                Preference::ShareClass(
                    "FIT1049".to_string(),
                    "Applied".to_string(),
                    "b".to_string(),
                ),
                Preference::ShareClass(
                    "FIT1047".to_string(),
                    "Applied".to_string(),
                    "a".to_string(),
                ),
            ],
        }),
        (Member {
            name: "d".to_string(),
            units: vec![
                "FIT1045".to_string(),
                "FIT1049".to_string(),
                "FIT1047".to_string(),
                "MTH1020".to_string(),
            ],
            preferences: vec![
                Preference::ShareClass("1045".to_string(), "Applied".to_string(), "a".to_string()),
                Preference::ShareClass("1045".to_string(), "Applied".to_string(), "b".to_string()),
                Preference::ShareClass(
                    "FIT1047".to_string(),
                    "Applied".to_string(),
                    "a".to_string(),
                ),
                Preference::ShareClass(
                    "FIT1049".to_string(),
                    "Applied".to_string(),
                    "c".to_string(),
                ),
            ],
        }),
    ];

    let solution = solve(&classes, &members);

    println!("{:#?}", solution.1);

    Ok(())
}
