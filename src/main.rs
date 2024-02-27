use anyhow::{Ok, Result};
use clap::Parser;
use std::io::{stdout, BufWriter};

/// Quicklines: efficiently sample lines from large files, with replacement
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input file
    #[arg()]
    file_path: String,

    /// Avoid selecting the same sentence multiple times (=sample without replacement).
    #[arg(long)]
    deduplicate: bool,

    /// Number of lines to sample
    #[arg(short, long)]
    count: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut writer = BufWriter::new(stdout());
    quicklines::quicklines(
        &args.file_path,
        args.count as usize,
        !args.deduplicate,
        &mut writer,
    )?;

    Ok(())
}
