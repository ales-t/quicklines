use anyhow::{Ok, Result};

use std::fs::File;
use std::io::Write;

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

/// Write roughly `count` lines from uniformly spaced parts of the input file.
///
/// Uses `mmap` to do this relatively efficiently.
pub fn quicklines<W: Write>(file_path: &str, mut count: usize, mut writer: W) -> Result<()> {
    let mmapped = mmap_file(file_path)?;
    let total_size = mmapped.len();

    if count > total_size {
        count = total_size; // we make at least steps of size 1, and at most `total_size` steps
    }

    let step_size = total_size / count;

    for i in 0..count {
        let offset = step_size * i;
        if let Some((begin, end)) = maybe_extract_line(&mmapped, offset) {
            if begin < step_size * (i + 1) {
                writer.write_all(&mmapped[begin..end])?;
            }
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
