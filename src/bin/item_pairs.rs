use std::{
    collections::{HashMap, HashSet},
    env::args_os,
    fs::{self, read_dir, DirEntry, File},
    io::{Cursor, Write},
    process::exit,
};

use anyhow::Context;
use rayon::prelude::*;
use smm2_stats::{course_decryptor, level_parser::Level};

fn main() -> anyhow::Result<()> {
    let mut args = args_os().skip(1);
    let input_dir = args.next().unwrap_or_else(usage);
    let output_path = args.next();

    let totals = read_dir(input_dir)
        .context("cannot read input dir")?
        .par_bridge()
        .map(|entry_result| match entry_result {
            Ok(x) => x,
            Err(error) => {
                eprintln!("cannot read input dir: {}", error);
                exit(1);
            }
        })
        .map(handle_entry)
        .flat_map(|result| match result {
            Ok(x) => Some(x),
            Err(error) => {
                eprintln!("cannot read input file: {}", error);
                None
            }
        })
        .reduce(|| HashMap::new(), merge_totals);

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

type Totals = HashMap<(&'static str, &'static str), u64>;

fn handle_entry(entry: DirEntry) -> anyhow::Result<Totals> {
    let encrypted_data = fs::read(entry.path())
        .with_context(|| format!("cannot read input file {:?}", entry.file_name()))?;
    let level_data = course_decryptor::decrypt_course_data(&encrypted_data);
    let level = Level::parse(&mut Cursor::new(level_data))
        .with_context(|| format!("failed to parse level from {:?}", entry.file_name()))?;

    let mut totals = HashMap::new();
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
    Ok(totals)
}

fn merge_totals(mut lhs: Totals, rhs: Totals) -> Totals {
    for (key, count) in rhs {
        *lhs.entry(key).or_insert(0) += count;
    }
    lhs
}
