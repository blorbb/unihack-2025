use anyhow::Result;
use backend::solver::ClassTimes;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let classes: ClassTimes = backend::classes::load_classes(Path::new("../class-data/classes"))
        .await?
        .into_iter()
        .map(|(a, (b, c))| (a, c))
        .collect();

    println!("{classes:?}");

    Ok(())
}
