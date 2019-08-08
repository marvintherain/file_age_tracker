use std::time::{SystemTime, Duration};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

#[derive(Debug)]
struct File {
    path: PathBuf,
    creation_date: SystemTime,
    time_since_creation: Duration,
}

impl File {
    fn new(path: PathBuf, creation_date: SystemTime) -> File {
        File {
            path,
            creation_date,
            time_since_creation: creation_date.elapsed().unwrap(),
        }
    }
}

fn main() {
    //to-do: tage path as command line param
    let dir: &Path = Path::new("C://users//danie//Desktop//testdir//");
    
    let walker = WalkDir::new(dir);
    let mut entries: Vec<File> = vec![];
    for entry in walker {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_dir() == false {
            let my_file = File::new(entry
                                        .path()
                                        .to_path_buf(), 
                                    entry
                                        .metadata()
                                        .unwrap()
                                        .created()
                                        .unwrap()
                                    );
            entries.push(my_file);
        }
    }

    println!("{:?}", entries);
}
