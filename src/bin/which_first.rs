use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{self, BufRead, BufReader},
    process::exit,
};

use anyhow::Context;
use smm2_stats::item_groups::get_group;

fn main() -> anyhow::Result<()> {
    let mut args = args().skip(1);
    let pairs_file = args.next().unwrap_or_else(usage);
    let group_a = args.next().unwrap_or_else(usage);
    let group_b = args.next().unwrap_or_else(usage);

    let items_a: Vec<&str> = get_group(&group_a);
    let items_b: Vec<&str> = get_group(&group_b);

    let pairs: HashMap<(String, String), u64> =
        BufReader::new(File::open(pairs_file).context("cannot open pairs file")?)
            .lines()
            .map(parse_pair_line)
            .collect::<Result<_, _>>()?;

    //FIXME: Groups are assumed "disjoint", i.e. they can never appear in the
    // same level. Example is "Update 2 Powerup" which has one powerup in each
    // game style.
    for &item_a1 in &items_a {
        for &item_a2 in &items_a {
            if item_a1 == item_a2 {
                continue;
            }
            match pairs.get(&(item_a1.into(), item_a2.into())) {
                Some(0) | None => {}
                _ => {
                    eprintln!(
                        "warn: items {} and {} in group 1 are not disjoint",
                        item_a1, item_a2
                    );
                }
            }
        }
    }
    for &item_b1 in &items_b {
        for &item_b2 in &items_b {
            if item_b1 == item_b2 {
                continue;
            }
            match pairs.get(&(item_b1.into(), item_b2.into())) {
                Some(0) | None => {}
                _ => {
                    eprintln!(
                        "warn: items {} and {} in group 2 are not disjoint",
                        item_b1, item_b2
                    );
                }
            }
        }
    }

    let all_a = items_a
        .iter()
        .flat_map(|&item| pairs.get(&(item.into(), item.into())))
        .sum::<u64>();
    let all_b = items_b
        .iter()
        .flat_map(|&item| pairs.get(&(item.into(), item.into())))
        .sum::<u64>();
    let a_before_b = items_a
        .iter()
        .flat_map(|&item_a| {
            items_b
                .iter()
                .flat_map(|&item_b| pairs.get(&(item_a.into(), item_b.into())))
        })
        .sum::<u64>();
    let b_before_a = items_a
        .iter()
        .flat_map(|&item_a| {
            items_b
                .iter()
                .flat_map(|&item_b| pairs.get(&(item_b.into(), item_a.into())))
        })
        .sum::<u64>();

    let a_wins = all_a - b_before_a;
    let b_wins = all_b - a_before_b;
    let total = a_wins + b_wins;

    println!("{} {} {} {}", all_a, a_before_b, b_before_a, all_b);

    println!(
        "{}: {:.1}%",
        group_a,
        100.0 * (a_wins as f32) / (total as f32)
    );
    println!(
        "{}: {:.1}%",
        group_b,
        100.0 * (b_wins as f32) / (total as f32)
    );

    Ok(())
}

fn usage<T>() -> T {
    eprintln!("usage: [pairs-file] [group-a] [group-b]");
    exit(1);
}

fn parse_pair_line(line_result: io::Result<String>) -> anyhow::Result<((String, String), u64)> {
    let line = line_result.context("cannot read pairs file")?;
    let mut fields = line.split(",");
    let item_a = fields.next().context("malformed pairs file")?.into();
    let item_b = fields.next().context("malformed pairs file")?.into();
    let count = fields
        .next()
        .context("malformed pairs file")?
        .parse()
        .context("cannot parse count")?;
    Ok(((item_a, item_b), count))
}
