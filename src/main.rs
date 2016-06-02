#[macro_use]
extern crate clap;
extern crate openssl;

mod file_comparer;

use std::fs;
use std::path::Path;


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

    let path1_iter = fs::read_dir(path1);
    let path1_iter = match path1_iter {
        Ok(rd) => rd,
        Err(err) => {
            println!("access {:?} failed with error:\n{}",
                     path1, err);
            return ;
        }
    };

    let path2_iter = fs::read_dir(path2);
    let path2_iter = match path2_iter {
        Ok(rd) => rd,
        Err(err) => {
            println!("access {:?} failed with error:\n{}",
                     path2, err);
            return ;
        }
    };
}
