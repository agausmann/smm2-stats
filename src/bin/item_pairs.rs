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

    let entries: Vec<_> = read_dir(input_dir)
        .and_then(|iter| iter.collect())
        .context("cannot read input dir")?;

    let totals = entries
        .par_chunks(256)
        .map(|entries| {
            let levels = entries.iter().flat_map(|entry| match handle_entry(entry) {
                Ok(x) => Some(x),
                Err(error) => {
                    eprintln!("cannot read file: {}", error);
                    None
                }
            });
            let mut totals = HashMap::new();
            for level in levels {
                count_items(&mut totals, &level)
            }
            totals
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

fn handle_entry(entry: &DirEntry) -> anyhow::Result<Level> {
    let encrypted_data = fs::read(entry.path())
        .with_context(|| format!("cannot read input file {:?}", entry.file_name()))?;
    let level_data = course_decryptor::decrypt_course_data(&encrypted_data);
    let level = Level::parse(&mut Cursor::new(level_data))
        .with_context(|| format!("failed to parse level from {:?}", entry.file_name()))?;
    Ok(level)
}

fn count_items(totals: &mut Totals, level: &Level) {
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

fn merge_totals(lhs: Totals, mut rhs: Totals) -> Totals {
    for (key, count) in lhs {
        *rhs.entry(key).or_insert(0) += count;
    }
    rhs
}
