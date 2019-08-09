use std::time::{SystemTime, Duration};
use std::path::{Path, PathBuf};
use std::fs;

use walkdir::WalkDir;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};

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

//--------------------------------------------------

fn main() {
    //to-do: take path as command line param
    let dir: &Path = Path::new("C://users//danie//Desktop//testdir//");
    
    let entries = read_folder_content(dir);

    //to-do: except comparison param from command line
    let compare_param = Duration::new(60000, 0);

    let filtered_entries = filter_files(entries, compare_param);
    
    let mut table = Table::new();

    println!("\nfiles that exceed creation date parameter");
    table.add_row(row!["path", "filename"]);
    for entry in &filtered_entries {
        table.add_row(row![entry.path.parent().unwrap().to_str().unwrap(),
            entry.path.file_name().unwrap().to_str().unwrap()
        ]);
    }
    table.printstd();
    // fs::remove_file(&filtered_entries[0].path).expect("Could not delete");
}

//--------------------------------------------------

fn filter_files(entries: Vec<File>, compare_param: Duration) -> Vec<File> {
    let filtered_entries: Vec<File> = entries
        .into_iter()
        .filter(|entry| entry.time_since_creation >= compare_param)
        .collect();
    
    filtered_entries
}

fn read_folder_content(path: &Path) -> Vec<File> {
        let walker = WalkDir::new(path);
        
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
        };
        
        entries    

}
