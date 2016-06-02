use std::path::Path;
use openssl::crypto::hash;
use std::fs::File;
use std::io::prelude::*;
use std::io;

pub fn file_compare<P: AsRef<Path>>(f1: P, f2: P) -> io::Result<bool>  {
    let mut f1 = try!(File::open(f1));
    let mut f2 = try!(File::open(f2));

    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();
    try!(f1.read_to_end(&mut buf1));
    try!(f2.read_to_end(&mut buf2));

    if buf1.len() != buf2.len() {
        return Ok(false);
    }

    let f1_md5 = hash(hash::Type::MD5, &buf1);
    let f2_md5 = hash(hash::Type::MD5, &buf2);
    if f1_md5 != f2_md5 {
        return Ok(false);
    }

    let f1_sha1 = hash(hash::Type::SHA1, &buf1);
    let f2_sha1 = hash(hash::Type::SHA1, &buf2);
    if f1_sha1 != f2_sha1 {
        return Ok(false);
    }

    Ok(true)
}

fn hash(t: hash::Type, data: &[u8]) -> Vec<u8> {
    let mut hash = hash::Hasher::new(t);
    let _ = hash.write_all(data);
    hash.finish()
}

#[cfg(test)]
mod tests {
    use file_comparer::file_compare;
    #[test]
    fn test_compare_same() {
        let result = file_compare("./Cargo.lock", "./Cargo.lock");
        assert!(result.unwrap());
    }

    #[test]
    fn test_compare_bad_path() {
        let result = file_compare("aaa", "bbb");
        assert_eq!(result.ok(), None);
    }

    #[test]
    fn test_compare_diff_path() {
        let result = file_compare("./Cargo.lock", "./Cargo.toml");
        assert!(result.unwrap(), false);
    }
}
