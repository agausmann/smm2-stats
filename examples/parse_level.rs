use std::io::Cursor;

use smm2_stats::{course_decryptor::decrypt_course_data, level_parser::Level};

fn main() {
    let mut args = std::env::args_os().skip(1);
    let infile = args.next().expect("missing argument [infile]");
    let encrypted = std::fs::read(infile).expect("cannot read input file");
    let decrypted = decrypt_course_data(&encrypted);
    let level = Level::parse(&mut Cursor::new(&decrypted)).unwrap();
    println!("{:?}", level.header.name);
    println!("{:?}", level.header.description);
}
