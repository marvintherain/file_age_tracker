use std::time::{SystemTime, Duration};
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

use walkdir::WalkDir;
use timeago;
use regex::Regex;
use clap::{Arg, App};

#[macro_use] extern crate prettytable;
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
    let matches = App::new("file age tracker")
        .version("0.1")
        .author("Daniel Markow")
        .about("Tracks your files age and then offers to delete or flag them")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .takes_value(true)
            .help("Add the path to the folder of file you want to track"))
        .get_matches();
    
    let path = matches.value_of("path");

    if let Some(s) = path {
    //to-do: implement logging

        let dir: &Path = Path::new(s);
        
        let entries = read_folder_content(dir);

        let compare_param = Duration::new(10, 0);

        let filtered_entries = filter_files(entries, compare_param);
        
        let mut table = Table::new();

        println!("\nfiles that exceed creation date parameter in folder {}\\", s.to_string());
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
            println!("should the listed files be flagged in filename (*_flagged.*)? yes / no");

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
