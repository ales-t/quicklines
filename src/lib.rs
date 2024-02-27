use anyhow::{Ok, Result};
use rand::Rng;

use std::io::Write;
use std::{collections::HashSet, fs::File};

use memchr::memchr;
use memmap2::{Mmap, MmapOptions};

fn mmap_file(file_path: &str) -> Result<Mmap> {
    let file = File::open(file_path)?;
    let mapped = unsafe { MmapOptions::new().map(&file) };
    Ok(mapped?)
}

fn consume_line(chunk: &[u8], offset: usize) -> Option<usize> {
    memchr(b'\n', &chunk[offset..]).map(|i| offset + i)
}

fn maybe_extract_line(chunk: &[u8], offset: usize) -> Option<(usize, usize)> {
    let is_start_of_line = offset == 0 || chunk[offset - 1] == b'\n';

    let begin = if is_start_of_line {
        offset
    } else {
        consume_line(chunk, offset)? + 1
    };

    let end = consume_line(chunk, begin)? + 1;

    if end <= chunk.len() {
        Some((begin, end))
    } else {
        None
    }
}

/// Write roughly `count` random lines from the input file.
///
/// Uses `mmap` to do this relatively efficiently.
pub fn quicklines<W: Write>(
    file_path: &str,
    count: usize,
    allow_duplicates: bool,
    mut writer: W,
) -> Result<()> {
    let mmapped = mmap_file(file_path)?;
    let total_size = mmapped.len();

    let mut covered_offsets = HashSet::new();
    for _ in 0..count {
        let offset = rand::thread_rng().gen_range(0..total_size);
        if let Some((begin, end)) = maybe_extract_line(&mmapped, offset) {
            if !allow_duplicates && !covered_offsets.insert(begin) {
                continue; // this is a duplicate
            }
            writer.write_all(&mmapped[begin..end])?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{consume_line, maybe_extract_line};
    #[test]
    fn consume_line_returns_correct_index() {
        let data = "newline\nand something else";
        assert_eq!(consume_line(data.as_bytes(), 0), Some(7));
    }

    #[test]
    fn consume_line_returns_none_when_not_found() {
        let data = "no newline here";
        assert_eq!(consume_line(data.as_bytes(), 0), None);
    }

    #[test]
    fn maybe_extract_line_starts_at_zero() {
        let data = "a single line\n";
        assert_eq!(
            maybe_extract_line(data.as_bytes(), 0),
            Some((0, data.len()))
        );
    }

    #[test]
    fn maybe_extract_line_starts_at_offset() {
        let data = "first\nsecond\n";
        assert_eq!(
            maybe_extract_line(data.as_bytes(), 6),
            Some((6, data.len()))
        );
    }

    #[test]
    fn maybe_extract_line_happy_case() {
        let data = "first line\nsecond line\n";
        assert_eq!(
            maybe_extract_line(data.as_bytes(), 5),
            Some((11, data.len()))
        );
    }
}
