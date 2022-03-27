use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{self, BufRead, BufReader},
    process::exit,
};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let mut args = args().skip(1);
    let pairs_file = args.next().unwrap_or_else(usage);
    let item_a = args.next().unwrap_or_else(usage);
    let item_b = args.next().unwrap_or_else(usage);

    let pairs: HashMap<(String, String), u64> =
        BufReader::new(File::open(pairs_file).context("cannot open pairs file")?)
            .lines()
            .map(parse_pair_line)
            .collect::<Result<_, _>>()?;

    let all_a = pairs
        .get(&(item_a.clone(), item_a.clone()))
        .copied()
        .unwrap_or(0);
    let all_b = pairs
        .get(&(item_b.clone(), item_b.clone()))
        .copied()
        .unwrap_or(0);
    let both = pairs
        .get(&(item_a.clone(), item_b.clone()))
        .copied()
        .unwrap_or(0);

    let only_a = all_a - both;
    let only_b = all_b - both;
    let total = only_a + only_b;

    println!("{} {} {}", all_a, both, all_b);

    println!(
        "{}: {:.1}%",
        item_a,
        100.0 * (only_a as f32) / (total as f32)
    );
    println!(
        "{}: {:.1}%",
        item_b,
        100.0 * (only_b as f32) / (total as f32)
    );

    Ok(())
}

fn usage<T>() -> T {
    eprintln!("usage: [pairs-file] [item-a] [item-b]");
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
