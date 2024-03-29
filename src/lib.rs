use anyhow::{anyhow, Ok, Result};

use std::io::Write;
use std::{collections::HashSet, fs::File};

use memchr::{memchr, memrchr};
use memmap2::Mmap;

/// Memory map a file. A tiny convenience wrapper around Mmap::new
fn mmap_file(file_path: &str) -> Result<Mmap> {
    let file = File::open(file_path)?;
    let mapped = unsafe { Mmap::map(&file) };
    Ok(mapped?)
}

/// Find the next newline and return its position.
fn find_newline(chunk: &[u8], offset: usize) -> Option<usize> {
    memchr(b'\n', &chunk[offset..]).map(|i| offset + i)
}

/// Find the first complete line from the given offset.
///
/// The function assumes that `offset == 0` is a valid line start.
fn maybe_extract_line(chunk: &[u8], offset: usize) -> Option<(usize, usize)> {
    let begin = if offset == 0 {
        0
    } else {
        // here we also handle the case where there is a newline just before
        // the current offset (so the line starts _exactly_ at `offset`)
        find_newline(chunk, offset - 1)? + 1
    };

    let end = find_newline(chunk, begin)? + 1;

    if end <= chunk.len() {
        Some((begin, end))
    } else {
        None
    }
}

/// Find the last valid position in the file where a full line can be found
fn last_valid_offset(chunk: &[u8]) -> Result<usize> {
    let end = memrchr(b'\n', chunk).ok_or(anyhow!("file has no newlines"))?;
    // we found an end (a newline character) so the file contains at least a single line (which starts at 0)
    match memrchr(b'\n', &chunk[..end]) {
        None => Err(anyhow!("file has no newlines")),
        Some(begin) => Ok(begin + 1), // point past the newline
    }
}

/// Sample with replacement and print the samples
fn sample_with_replacement<W: Write>(
    chunk: &[u8],
    count: usize,
    last_offset: usize,
    writer: &mut W,
) -> Result<()> {
    for _ in 0..count {
        let offset = fastrand::usize(0..last_offset + 1);
        let (begin, end) = maybe_extract_line(chunk, offset).ok_or(anyhow!("internal error"))?;
        writer.write_all(&chunk[begin..end])?;
    }

    Ok(())
}

/// Sample without replacement and print the samples
fn sample_without_replacement<W: Write>(
    chunk: &[u8],
    count: usize,
    last_offset: usize,
    writer: &mut W,
) -> Result<()> {
    let mut covered_offsets = HashSet::new();
    let mut extracted_size = 0;

    for i in 0..count {
        if extracted_size > last_offset {
            return Err(anyhow!(
                "cannot sample {} unique lines from file with only {} lines",
                count,
                i
            ));
        }

        loop {
            let offset = fastrand::usize(0..last_offset + 1);
            let (begin, end) =
                maybe_extract_line(chunk, offset).ok_or(anyhow!("internal error"))?;
            if covered_offsets.insert(begin) {
                extracted_size += end - begin;
                writer.write_all(&chunk[begin..end])?;
                break;
            }
        }
    }

    Ok(())
}

/// Write `count` random lines from the input file.
///
/// Uses `mmap` to do this relatively efficiently.
///
/// # Arguments
///
/// * `file_path` - path to the file to sample
/// * `count` - how many samples to take
/// * `allow_duplicates` - allow duplicate samples, i.e. sample with replacement
/// * `seed` - optional random seed to ensure deterministic behavior
/// * `writer` - where to write outputs
pub fn quicklines<W: Write>(
    file_path: &str,
    count: usize,
    allow_duplicates: bool,
    seed: Option<u64>,
    writer: &mut W,
) -> Result<()> {
    let mmapped = mmap_file(file_path)?;
    let last_offset = last_valid_offset(&mmapped)?;

    if let Some(seed_value) = seed {
        fastrand::seed(seed_value);
    }

    if allow_duplicates {
        sample_with_replacement(&mmapped, count, last_offset, writer)?;
    } else {
        sample_without_replacement(&mmapped, count, last_offset, writer)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{find_newline, maybe_extract_line};
    #[test]
    fn find_newline_returns_correct_index() {
        let data = "newline\nand something else";
        assert_eq!(find_newline(data.as_bytes(), 0), Some(7));
    }

    #[test]
    fn find_newline_returns_none_when_not_found() {
        let data = "no newline here";
        assert_eq!(find_newline(data.as_bytes(), 0), None);
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
