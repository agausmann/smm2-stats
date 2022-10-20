use std::{
    collections::{HashMap, HashSet},
    env::args_os,
    fs::File,
    io::Write,
    process::exit,
    time::Instant,
};

use anyhow::Context;
use smm2_stats::level_iter;

fn main() -> anyhow::Result<()> {
    let mut args = args_os().skip(1);
    let input_dir = args.next().unwrap_or_else(usage);
    let output_path = args.next();

    let start_time = Instant::now();

    let mut totals: HashMap<(&str, &str), u64> = HashMap::new();
    let mut num_levels = 0;

    level_iter::for_each_in(&input_dir, |level| {
        num_levels += 1;

        let mut items: HashSet<&str> = level
            .overworld
            .objects
            .iter()
            .chain(&level.subworld.objects)
            .flat_map(|obj| obj.name(level.header.game_style))
            .collect();

        // NOTE - Icicles are not in objects list:
        if level.overworld.icicles.len() + level.subworld.icicles.len() > 0 {
            items.insert("Icicle");
        }

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
    });

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
        num_levels as f32 / elapsed
    );

    Ok(())
}

fn usage<T>() -> T {
    eprintln!("usage: [levels-dir]");
    exit(1);
}
