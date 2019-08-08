use std::time::{SystemTime, Duration};
use std::path::PathBuf;

use walkdir::WalkDir;

#[derive(Debug)]
struct File {
    path: PathBuf,
    creation_date: SystemTime,
    time_since_creation: Duration,
}

fn main() {
    println!("Hello, world!");
}
