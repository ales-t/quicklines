use anyhow::{Ok, Result};
use clap::Parser;
use std::io::{stdout, BufWriter};

/// Quicklines
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to file
    #[arg()]
    file_path: String,

    /// Number of lines to sample
    #[arg(short, long)]
    count: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut writer = BufWriter::new(stdout());
    quicklines::quicklines(&args.file_path, args.count as usize, &mut writer)?;

    Ok(())
}
