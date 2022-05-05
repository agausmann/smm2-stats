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

    let groups = args.collect::<Vec<_>>();
    let group_items = groups
        .iter()
        .map(|group| get_group(&group))
        .collect::<Vec<_>>();

    let pairs: HashMap<(String, String), u64> =
        BufReader::new(File::open(pairs_file).context("cannot open pairs file")?)
            .lines()
            .map(parse_pair_line)
            .collect::<Result<_, _>>()?;

    //FIXME: Groups are assumed "disjoint", i.e. they can never appear in the
    // same level. Example is "Update 2 Powerup" which has one powerup in each
    // game style.
    for items in &group_items {
        for &item_a1 in items {
            for &item_a2 in items {
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
    }

    let totals = group_items
        .iter()
        .map(|items| {
            items
                .iter()
                .flat_map(|&item| pairs.get(&(item.into(), item.into())))
                .sum::<u64>()
        })
        .collect::<Vec<u64>>();

    let grand_total = totals.iter().sum::<u64>();
    //TODO account for variance (items commonly seen together)
    // - do we need data on more than just pairs (triples etc) to do this?

    for (group, total) in groups.iter().zip(&totals) {
        println!(
            "{}: {:.1} %",
            group,
            *total as f32 / grand_total as f32 * 100.0
        );
    }

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
