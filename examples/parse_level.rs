use std::io::Cursor;

use smm2_stats::{course_decryptor::decrypt_course_data, level_parser::Level};

fn main() {
    let mut args = std::env::args_os().skip(1);
    let infile = args.next().expect("missing argument [infile]");
    let encrypted = std::fs::read(infile).expect("cannot read input file");
    let decrypted = decrypt_course_data(&encrypted);
    let level = Level::parse(&mut Cursor::new(&decrypted)).unwrap();

    println!("Name: {:?}", level.header.name);
    println!("Description: {:?}", level.header.description);
    println!(
        "Game style: {}",
        level.header.game_style_str().unwrap_or("Unknown")
    );

    println!("Overworld objects: {}", level.overworld.objects.len());
    for obj in &level.overworld.objects {
        println!(
            "    {},{}: {}",
            obj.x,
            obj.y,
            obj.name().unwrap_or("Unknown"),
        )
    }

    println!("Subworld objects: {}", level.subworld.objects.len());
    for obj in &level.subworld.objects {
        println!(
            "    {},{}: {}",
            obj.x,
            obj.y,
            obj.name().unwrap_or("Unknown"),
        )
    }
}
