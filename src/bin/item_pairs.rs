use std::{
    collections::{HashMap, HashSet},
    env::args_os,
    fs::{self, read_dir, DirEntry, File},
    io::{self, Cursor, Write},
    process::exit,
};

use anyhow::Context;
use smm2_stats::{course_decryptor, level_parser::Level};

fn main() -> anyhow::Result<()> {
    let mut args = args_os().skip(1);
    let input_dir = args.next().unwrap_or_else(usage);
    let output_path = args.next();

    let mut totals: HashMap<(&str, &str), u64> = HashMap::new();

    for entry_result in read_dir(input_dir).context("cannot read input dir")? {
        match handle_entry(entry_result) {
            Ok(level) => {
                let items: HashSet<&str> = level
                    .overworld
                    .objects
                    .iter()
                    .chain(&level.subworld.objects)
                    .flat_map(|obj| obj.name(level.header.game_style))
                    .collect();

                for &item in &items {
                    *totals.entry((item, item)).or_insert(0) += 1;
                }
                for &item_a in &items {
                    for &item_b in &items {
                        if item_a != item_b {
                            *totals.entry((item_a, item_b)).or_insert(0) += 1;
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("{:#?}", error);
            }
        }
    }

    let mut totals: Vec<_> = totals.into_iter().collect();
    totals.sort();

    if let Some(output_path) = output_path {
        let mut output_file = File::create(output_path).context("cannot create output file")?;
        for ((a, b), count) in totals {
            writeln!(output_file, "{},{},{}", a, b, count)
                .context("cannot write to output file")?;
        }
    } else {
        for ((a, b), count) in totals {
            println!("{},{},{}", a, b, count);
        }
    }

    Ok(())
}

fn usage<T>() -> T {
    eprintln!("usage: [levels-dir]");
    exit(1);
}

fn handle_entry(entry_result: io::Result<DirEntry>) -> anyhow::Result<Level> {
    let entry = entry_result.context("cannot read input dir")?;
    let encrypted_data = fs::read(entry.path())
        .with_context(|| format!("cannot read input file {:?}", entry.file_name()))?;
    let level_data = course_decryptor::decrypt_course_data(&encrypted_data);
    let level = Level::parse(&mut Cursor::new(level_data))
        .with_context(|| format!("failed to parse level from {:?}", entry.file_name()))?;
    Ok(level)
}
