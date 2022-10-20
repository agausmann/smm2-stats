//! Utility to try and correlate the existence of object IDs with other
//! conditions in level data.
//!
//! Original motivation: Icicle was not being found as an object; despite being
//! labeled in other level parsers as object ID 118, I have never found a level
//! with that object ID in it. So, I began a hunt to find which object ID should
//! be the "icicle" ID (spoiler alert: there is none).
//!
//! There is another way to know whether an icicle is in a level, which is by
//! checking the size of the `icicles` list. So, the approach to finding an
//! appropriate object ID is to find the correlation between two random
//! variables: whether a level satisfies the predicate as specified above, and
//! for each object ID, whether the level contains it. Any related IDs should
//! have the highest correlations (and should be 100%, if there is only one
//! related ID).

use std::{
    collections::{HashMap, HashSet},
    env::args_os,
    fs::{self, read_dir, DirEntry},
    io::Cursor,
    process::exit,
    time::Instant,
};

use anyhow::Context;
use smm2_stats::level_parser::Level;

fn level_filter(level: &Level) -> bool {
    level.overworld.icicles.len() + level.subworld.icicles.len() > 0
    // level.overworld.creepers.len() + level.subworld.creepers.len() > 0
    // level.overworld.track_blocks.len() + level.subworld.track_blocks.len() > 0
}

fn main() -> anyhow::Result<()> {
    let mut args = args_os().skip(1);
    let input_dir = args.next().unwrap_or_else(usage);

    let start_time = Instant::now();

    let entries: Vec<_> = read_dir(input_dir)
        .and_then(|iter| iter.collect::<Result<_, _>>())
        .context("cannot read input dir")?;
    let mut item_counts: HashMap<i16, usize> = HashMap::new();
    let mut total_levels: usize = 0;
    let mut total_no: usize = 0;

    for entry in &entries {
        match handle_entry(entry) {
            Ok(level) => {
                let items: HashSet<_> = level
                    .overworld
                    .objects
                    .iter()
                    .chain(&level.subworld.objects)
                    .map(|obj| obj.id)
                    .collect();

                if level_filter(&level) {
                    for &item in &items {
                        *item_counts.entry(item).or_insert(total_no) += 1;
                    }
                } else {
                    for &item in &items {
                        item_counts.entry(item).or_insert(total_no);
                    }
                    for (item, count) in item_counts.iter_mut() {
                        if !items.contains(&item) {
                            *count += 1;
                        }
                    }
                    total_no += 1;
                }

                total_levels += 1;

                if total_levels % 100 == 0 {
                    let now = Instant::now();
                    println!();
                    println!(
                        "n = {}, ({:.1}/s)",
                        total_levels,
                        total_levels as f32 / (now - start_time).as_secs_f32()
                    );

                    println!("Best matches:");
                    let mut entries: Vec<_> = item_counts.iter().map(|(&k, &v)| (v, k)).collect();
                    entries.sort();
                    for (count, id) in entries.into_iter().rev().take(10) {
                        println!(
                            "{} ({:.1}%) id={}", // flag={:#08x}",
                            count,
                            count as f32 / total_levels as f32 * 100.0,
                            id,
                        );
                    }
                }
            }
            Err(error) => {
                eprintln!("{:#?}", error);
            }
        }
    }

    let finish_time = Instant::now();

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

fn handle_entry(entry: &DirEntry) -> anyhow::Result<Level> {
    let level_data = fs::read(entry.path())
        .with_context(|| format!("cannot read input file {:?}", entry.file_name()))?;
    let level = Level::parse(&mut Cursor::new(level_data))
        .with_context(|| format!("failed to parse level from {:?}", entry.file_name()))?;
    Ok(level)
}
