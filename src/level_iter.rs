use std::{
    fs::{read_dir, File},
    path::Path,
};

use crate::{archive::Archive, level_parser::Level};

pub fn for_each_in<P, F>(path: &P, mut visitor: F)
where
    P: AsRef<Path>,
    F: FnMut(Level),
{
    let path = path.as_ref();

    if path.is_dir() {
        let dir_iter = match read_dir(path) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("cannot read {:?}: {}", path, err);
                return;
            }
        };
        for entry_result in dir_iter {
            let entry = match entry_result {
                Ok(x) => x,
                Err(err) => {
                    eprintln!("cannot read {:?}: {}", path, err);
                    continue;
                }
            };
            let mut file = match File::open(entry.path()) {
                Ok(x) => x,
                Err(err) => {
                    eprintln!("cannot open {:?}: {}", entry.path(), err);
                    continue;
                }
            };
            let level = match Level::parse(&mut file) {
                Ok(x) => x,
                Err(err) => {
                    eprintln!("cannot parse level from {:?}: {}", entry.path(), err);
                    continue;
                }
            };
            visitor(level);
        }
    } else {
        let file = match File::open(path) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("cannot open {:?}: {}", path, err);
                return;
            }
        };
        let mut archive = Archive::new(file);
        let mut reader = match archive.read() {
            Ok(x) => x,
            Err(err) => {
                eprintln!("cannot read {:?}: {}", path, err);
                return;
            }
        };
        while let Some(result) = reader.next_level() {
            let level = match result {
                Ok(x) => x,
                Err(err) => {
                    eprintln!("cannot parse level: {}", err);
                    continue;
                }
            };
            visitor(level);
        }
    }
}
