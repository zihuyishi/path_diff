use std::fs::{read_dir, ReadDir, DirEntry};
use std::ffi::OsString;
use std::path::Path;
use std::io::Result;

pub enum CompareResult {
    LeftHas,
    RightHas,
    Diff,
    Same,
}

pub struct CompareEntry {
    result: CompareResult,
    path: OsString,
}

pub fn path_compare(path1: &Path, path2: &Path) -> Result<Vec<CompareEntry>> {
    let mut results = Vec::new();
    let rd1 = try!(read_dir(path1));
    let rd2 = try!(read_dir(path2));

    let rd1: Vec<DirEntry> = rd1.filter_map(|dir| dir.ok()).collect();
    let rd2: Vec<DirEntry> = rd2.filter_map(|dir| dir.ok()).collect();
    let both: Vec<OsString> = rd1.iter()
                                 .fold(Vec::new(), |v, item| {
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
                                              }
                                          })
                                          .collect();
    results.append(&mut left);
    let mut right: Vec<CompareEntry> = right.into_iter()
                                            .map(|item| {
                                                CompareEntry {
                                                    result: CompareResult::RightHas,
                                                    path: item.path().into_os_string(),
                                                }
                                            })
                                            .collect();
    results.append(&mut right);

    // compare same part
    Ok(results)
}
