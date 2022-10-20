use std::{
    fs::{read_dir, File},
    path::Path,
};

use anyhow::Context;
use smm2_stats::archive::ArchiveWriter;

fn main() -> anyhow::Result<()> {
    let input_dir = Path::new("levels/sexpert/");
    let mut output = ArchiveWriter::new(File::create("levels/sexpert.tar.gz")?);

    let entries: Vec<_> = read_dir(input_dir)
        .and_then(|iter| iter.collect::<Result<_, _>>())
        .context("cannot read input dir")?;

    for (i, entry) in entries.iter().enumerate().take(1000) {
        if i % 100 == 0 {
            println!("{}", i);
        }
        let contents = std::fs::read(entry.path())?;
        output
            .append_decrypted_level(entry.path().file_name().unwrap(), &mut contents.as_slice())?;
    }
    output.finish()?;

    Ok(())
}
