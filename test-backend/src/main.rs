use std::path::Path;

#[tokio::main]
async fn main() {
    let classes = backend::server::classes::load_classes(Path::new("../class-data/classes")).await;
    println!("{classes:?}");
}
