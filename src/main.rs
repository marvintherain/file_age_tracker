use std::time::{SystemTime, Duration};
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

use walkdir::WalkDir;
use timeago;
use regex::Regex;


#[macro_use] extern crate prettytable;
// use prettytable::{Table, Row, Cell};
use prettytable::{Table};

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

    fn get_path(&self) -> String {
        self.path.parent().unwrap().to_str().unwrap().replace("//", "/")
    }

    fn get_file_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    fn get_time_since_creation(&self) -> String {
        let f = timeago::Formatter::new();
        f.convert(self.time_since_creation)
    }
}

//--------------------------------------------------

fn main() {
    //to-do: take path as command line param
    //to-do: implement logging
    let dir: &Path = Path::new("C://users//danie//Desktop//Scans - Kopie//");
    
    let entries = read_folder_content(dir);

    //to-do: except comparison param from command line
    let compare_param = Duration::new(10, 0);

    let filtered_entries = filter_files(entries, compare_param);
    
    let mut table = Table::new();

    println!("\nfiles that exceed creation date parameter");
    table.add_row(row!["path", "filename", "time since creation"]);
    for entry in &filtered_entries {
        table.add_row(row![entry.get_path(),
            entry.get_file_name(),
            entry.get_time_since_creation()
        ]);

    }
    table.printstd();
    println!("\ndo you want to delete the files listed? yes / no
(deleted files will not appear in trash!)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("line could not be read");

    let re_yes = Regex::new(r"\s?yes\s?").unwrap();
    let re_no = Regex::new(r"\s?no\s?").unwrap();

    if re_yes.is_match(&input) == true {
        delete_files(filtered_entries);
    } else if re_no.is_match(&input) {
        println!("--> no action taken");
        println!("should the listed files be flagged in filename (*_flagged.*)?");

        io::stdin().read_line(&mut input).expect("line could not be read");
        
        if re_yes.is_match(&input) == true {
            flag_files(filtered_entries);
            println!("--> files have been flagged");
        } else if re_no.is_match(&input) {
            println!("--> no action taken");
        }
    }
    else {
        println!("--> command not found");
    };

}

//--------------------------------------------------

fn flag_files(entries: Vec<File>) {
    for entry in entries {
        fs::rename(&entry.path, &entry.path.to_str().unwrap().replace(".", "_flagged.")).unwrap();
    }
}

fn delete_files(entries: Vec<File>) {
    for entry in entries {
        fs::remove_file(entry.path)
            .expect("deletion failed");
    }; 

    println!("--> listed files were deleted");
}

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
