use std::{collections::HashMap, env::args_os, process::exit, time::Instant};

use smm2_stats::level_iter;

fn main() -> anyhow::Result<()> {
    let mut args = args_os().skip(1);
    let input_dir = args.next().unwrap_or_else(usage);

    let start_time = Instant::now();

    let mut mainworld_totals: HashMap<&str, u64> = HashMap::new();
    let mut subworld_totals: HashMap<&str, u64> = HashMap::new();
    let mut num_levels = 0;

    level_iter::for_each_in(&input_dir, |level| {
        num_levels += 1;

        if let Some(theme) = level.overworld.map_header.theme_str() {
            *mainworld_totals.entry(theme).or_insert(0) += 1;
        }
        if let Some(theme) = level.subworld.map_header.theme_str() {
            *subworld_totals.entry(theme).or_insert(0) += 1;
        }
    });

    let mut mainworld_totals: Vec<_> = mainworld_totals.into_iter().collect();
    let mut subworld_totals: Vec<_> = subworld_totals.into_iter().collect();
    mainworld_totals.sort_by_key(|(_name, count)| *count);
    subworld_totals.sort_by_key(|(_name, count)| *count);

    let finish_time = Instant::now();

    println!("Main world:");
    for (name, count) in mainworld_totals {
        println!(
            "{:<12} {:>6} ({:>5.2}%)",
            name,
            count,
            (count as f32) / (num_levels as f32) * 100.0
        );
    }
    println!();

    println!("Subworld:");
    for (name, count) in subworld_totals {
        println!(
            "{:<12} {:>6} ({:>5.2}%)",
            name,
            count,
            (count as f32) / (num_levels as f32) * 100.0
        );
    }
    println!();

    let elapsed = (finish_time - start_time).as_secs_f32();
    eprintln!(
        "took {:.3} seconds ({:.1} per second)",
        elapsed,
        num_levels as f32 / elapsed
    );

    Ok(())
}

fn usage<T>() -> T {
    eprintln!("usage: [levels-dir]");
    exit(1);
}
