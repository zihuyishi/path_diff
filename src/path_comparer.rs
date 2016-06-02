use std::fs::{read_dir, DirEntry};
use std::ffi::OsString;
use std::path::Path;
use std::io::Result;
use std::fmt::{Display, Formatter};
use std::fmt;
use file_comparer::file_compare;

pub enum CompareResult {
    LeftHas,
    RightHas,
    Diff,
}

pub struct CompareEntry {
    pub result: CompareResult,
    pub path: OsString,
    pub ano_path: Option<OsString>,
}

impl Display for CompareEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.result {
            CompareResult::LeftHas => writeln!(f, "@- {}\n", self.path.to_str().unwrap()),
            CompareResult::RightHas => writeln!(f, "@+ {}\n", self.path.to_str().unwrap()),
            CompareResult::Diff => writeln!(f, "@diff\n{}\n{}\n",
                                            self.path.to_str().unwrap(),
                                            self.ano_path.as_ref().unwrap().to_str().unwrap()),
        }
    }
}

pub fn path_compare(path1: &Path, path2: &Path) -> Result<Vec<CompareEntry>> {
    let mut results = Vec::new();
    let rd1 = try!(read_dir(path1));
    let rd2 = try!(read_dir(path2));

    let rd1: Vec<DirEntry> = rd1.filter_map(|dir| dir.ok()).collect();
    let rd2: Vec<DirEntry> = rd2.filter_map(|dir| dir.ok()).collect();
    let both: Vec<OsString> = rd1.iter()
                                 .fold(Vec::new(), |mut v, item| {
                                     if rd2.iter()
                                           .any(|file| file.file_name() == item.file_name()) {
                                         v.push(item.file_name());
                                     };
                                     v
                                 });
    let left: Vec<DirEntry> = rd1.into_iter()
                                 .filter(|item| {
                                     both.iter().all(|name| name != item.file_name().as_os_str())
                                 })
                                 .collect();
    let right: Vec<DirEntry> = rd2.into_iter()
                                  .filter(|item| {
                                      both.iter().all(|name| name != item.file_name().as_os_str())
                                  })
                                  .collect();
    // take left and right into results
    let mut left: Vec<CompareEntry> = left.into_iter()
                                          .map(|item| {
                                              CompareEntry {
                                                  result: CompareResult::LeftHas,
                                                  path: item.path().into_os_string(),
                                                  ano_path: None,
                                              }
                                          })
                                          .collect();
    results.append(&mut left);
    let mut right: Vec<CompareEntry> = right.into_iter()
                                            .map(|item| {
                                                CompareEntry {
                                                    result: CompareResult::RightHas,
                                                    path: item.path().into_os_string(),
                                                    ano_path: None,
                                                }
                                            })
                                            .collect();
    results.append(&mut right);

    // compare same part
    for name in both {
        let mut left_path = path1.to_path_buf();
        left_path.push(name.as_os_str());
        let mut right_path = path2.to_path_buf();
        right_path.push(name.as_os_str());

        if left_path.is_dir() && right_path.is_dir() {
            let mut sub = path_compare(left_path.as_path(), right_path.as_path());
            if let Ok(ref mut sub) = sub {
                results.append(sub);
            }
        }
        else if left_path.is_file() && right_path.is_file() {
            let compare_result = file_compare(left_path.as_path(), right_path.as_path())
                .unwrap_or(false);
            if !compare_result {
                let result = CompareEntry {
                    result: CompareResult::Diff,
                    path: left_path.into_os_string(),
                    ano_path: Some(right_path.into_os_string()),
                };
                results.push(result);
            }
        }
        else {
            let result = CompareEntry {
                result: CompareResult::Diff,
                path: left_path.into_os_string(),
                ano_path: Some(right_path.into_os_string()),
            };
            results.push(result);
        }
    }
    Ok(results)
}
