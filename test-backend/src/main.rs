use anyhow::Result;
use backend::server::classes::*;
use backend::server::solver::{ClassTimes, solve};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let classes: ClassTimes =
        backend::server::classes::load_classes(Path::new("../class-data/classes"))
            .await?
            .into_iter()
            .map(|(a, (b, c))| (a, c))
            .collect();

    println!("{classes:?}");

    Ok(())
}
