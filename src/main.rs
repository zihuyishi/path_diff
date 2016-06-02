#[macro_use]
extern crate clap;
extern crate openssl;

mod path_comparer;
pub mod file_comparer;

use std::path::Path;
use path_comparer::{CompareEntry, path_compare};

fn main() {
    let matches = clap_app!(differ_path  => 
        (version: "1.0")
        (author: "Zihuyishi. <zihuyishi@live.cn>")
        (about: "Show diffirent between two path")
        (@arg PATH1: +required "one path")
        (@arg PATH2: +required "another path")
    ).get_matches();

    let path1 = matches.value_of("PATH1").unwrap();
    let path2 = matches.value_of("PATH2").unwrap();
    let path1 = Path::new(path1);
    let path2 = Path::new(path2);

    if !path1.is_dir() {
        println!("{:?} is not a folder", path1);
        return ;
    }
    if !path2.is_dir() {
        println!("{:?} is not a folder", path2);
        return ;
    }
    let results = path_compare(&path1, &path2);
    match results {
        Ok(results) => print_results(&results),
        Err(err) => println!("compare with error:\n{}", err),
    };
        
}

fn print_results(results: &Vec<CompareEntry>) {
    for entry in results {
        println!("{}", entry);
    }
}
