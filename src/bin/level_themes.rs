use std::{collections::HashMap, env::args_os, process::exit, time::Instant};

use smm2_stats::level_iter;

fn main() -> anyhow::Result<()> {
    let mut args = args_os().skip(1);
    let input_dir = args.next().unwrap_or_else(usage);

    let start_time = Instant::now();

    let mut totals: HashMap<&str, u64> = HashMap::new();
    let mut num_levels = 0;

    level_iter::for_each_in(&input_dir, |level| {
        num_levels += 1;

        if let Some(theme) = level.overworld.map_header.theme_str() {
            *totals.entry(theme).or_insert(0) += 1;
        }
    });

    let mut totals: Vec<_> = totals.into_iter().collect();
    totals.sort_by_key(|(_name, count)| *count);

    let finish_time = Instant::now();

    for (name, count) in totals {
        println!(
            "{:<12} {:>6} ({:>5.2}%)",
            name,
            count,
            (count as f32) / (num_levels as f32) * 100.0
        );
    }

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
