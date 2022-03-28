use std::{
    collections::HashMap,
    env::args_os,
    fs::{self, read_dir, File},
    io::{Cursor, Write},
    path::Path,
    process::exit,
    time::Instant,
};

use anyhow::Context;
use smm2_stats::level_parser::Level;

fn main() -> anyhow::Result<()> {
    let mut args = args_os().skip(1);
    let input_dir = args.next().unwrap_or_else(usage);
    let output_path = args.next();

    let start_time = Instant::now();

    let entries: Vec<_> = read_dir(input_dir)
        .and_then(|iter| iter.collect::<Result<_, _>>())
        .context("cannot read input dir")?;
    let mut totals: HashMap<(&str, &str), u64> = HashMap::new();

    for entry in &entries {
        let level = match load_level(&entry.path()) {
            Ok(x) => x,
            Err(error) => {
                eprintln!("{:#?}", error);
                continue;
            }
        };
        let mut item_positions: HashMap<&str, i32> = HashMap::new();

        for object in level
            .overworld
            .objects
            .iter()
            .chain(&level.subworld.objects)
        {
            if let Some(name) = object.name(level.header.game_style) {
                let slot = item_positions.entry(name).or_insert(i32::MAX);
                *slot = object.x.min(*slot);
            }
        }

        for &item in item_positions.keys() {
            *totals.entry((item, item)).or_insert(0) += 1;
        }
        for (&a, &ax) in item_positions.iter() {
            for (&b, &bx) in item_positions.iter() {
                if a != b && ax <= bx {
                    *totals.entry((a, b)).or_insert(0) += 1;
                }
            }
        }
    }

    let mut totals: Vec<_> = totals.into_iter().collect();
    totals.sort();
    let finish_time = Instant::now();

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

    let elapsed = (finish_time - start_time).as_secs_f32();
    eprintln!(
        "took {:.3} seconds ({:.1} per second)",
        elapsed,
        entries.len() as f32 / elapsed
    );

    Ok(())
}

fn usage<T>() -> T {
    eprintln!("usage: [levels-dir]");
    exit(1);
}

fn load_level(path: &Path) -> anyhow::Result<Level> {
    let level_data =
        fs::read(path).with_context(|| format!("cannot read input file {:?}", path))?;
    let level = Level::parse(&mut Cursor::new(level_data))
        .with_context(|| format!("failed to parse level from {:?}", path))?;
    Ok(level)
}
