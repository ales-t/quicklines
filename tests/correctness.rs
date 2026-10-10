mod support;

use std::collections::HashSet;
use std::io::{self, Write};

use quicklines::quicklines;
use support::{assert_records, records, sample, TestFile};

#[test]
fn zero_samples_succeed_for_empty_and_nonempty_files() {
    for bytes in [b"".as_slice(), b"one\n", b"one\ntwo\n"] {
        let input = TestFile::new(bytes);
        for allow_duplicates in [false, true] {
            assert!(sample(&input, 0, allow_duplicates, 42).is_empty());
        }
    }
}

#[test]
fn positive_sample_from_empty_input_returns_an_error() {
    let input = TestFile::new(b"");
    for allow_duplicates in [false, true] {
        let mut output = Vec::new();
        assert!(quicklines(input.path(), 1, allow_duplicates, Some(42), &mut output).is_err());
        assert!(output.is_empty());
    }
}

#[test]
fn one_line_can_be_selected_with_and_without_repeats() {
    let input = TestFile::new(b"one\n");
    assert_eq!(sample(&input, 1, false, 42), b"one\n");
    assert_eq!(sample(&input, 10, true, 42), b"one\n".repeat(10));
}

#[test]
fn a_line_without_a_final_newline_gets_an_output_newline() {
    let input = TestFile::new(b"one");
    assert_eq!(sample(&input, 1, false, 42), b"one\n");
    assert_eq!(sample(&input, 10, true, 42), b"one\n".repeat(10));
}

#[test]
fn a_final_line_without_a_newline_is_included_in_a_complete_sample() {
    let input = TestFile::new(b"first\nsecond\nlast");
    let output = sample(&input, 3, false, 42);
    assert_records(&output, 3, &[b"first\n", b"second\n", b"last\n"]);
    assert_eq!(
        records(&output).into_iter().collect::<HashSet<_>>().len(),
        3
    );
}

#[test]
fn an_empty_line_is_a_record_but_a_final_newline_is_not_an_extra_record() {
    let input = TestFile::new(b"\n");
    assert_eq!(sample(&input, 1, false, 42), b"\n");
    assert_eq!(sample(&input, 4, true, 42), b"\n\n\n\n");
    let mut output = Vec::new();
    assert!(quicklines(input.path(), 2, false, Some(42), &mut output).is_err());
    assert!(output.is_empty());
}

#[test]
fn complete_unique_samples_include_every_position_for_unequal_line_lengths() {
    for bytes in [
        b"a\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\n".as_slice(),
        b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\na\n",
        b"a\nbbb\ncccccccccccccccc\ndd\n",
    ] {
        let input = TestFile::new(bytes);
        let expected = records(bytes);
        for seed in 0..32 {
            let output = sample(&input, expected.len(), false, seed);
            assert_records(&output, expected.len(), &expected);
            let actual = records(&output).into_iter().collect::<HashSet<_>>();
            assert_eq!(actual, expected.iter().copied().collect(), "seed {seed}");
        }
    }
}

#[test]
fn unique_selection_uses_positions_not_distinct_text() {
    let input = TestFile::new(b"same\nsame\nelse\n");
    let output = sample(&input, 3, false, 42);
    assert_records(&output, 3, &[b"same\n", b"else\n"]);
    assert_eq!(
        records(&output)
            .iter()
            .filter(|&&line| line == b"same\n")
            .count(),
        2
    );
}

#[test]
fn an_impossible_unique_request_does_not_write_partial_output() {
    let input = TestFile::new(b"one\ntwo\n");
    let mut output = Vec::new();
    assert!(quicklines(input.path(), 3, false, Some(42), &mut output).is_err());
    assert!(
        output.is_empty(),
        "check the line count before writing output"
    );
}

#[test]
fn a_unique_subset_has_the_requested_count_and_no_repeated_positions() {
    let input = TestFile::new(b"aa\nbb\ncc\ndd\nee\n");
    for seed in 0..32 {
        let output = sample(&input, 3, false, seed);
        assert_records(&output, 3, &[b"aa\n", b"bb\n", b"cc\n", b"dd\n", b"ee\n"]);
        assert_eq!(
            records(&output).into_iter().collect::<HashSet<_>>().len(),
            3
        );
    }
}

#[test]
fn selection_with_repeats_can_exceed_the_input_line_count() {
    let input = TestFile::new(b"aa\nbb\ncc\n");
    let output = sample(&input, 100, true, 42);
    assert_records(&output, 100, &[b"aa\n", b"bb\n", b"cc\n"]);
    assert!(records(&output).into_iter().collect::<HashSet<_>>().len() < 100);
}

#[test]
fn blank_crlf_utf8_and_non_utf8_lines_keep_their_bytes() {
    for bytes in [
        b"\n\n\n".as_slice(),
        b"first\r\nsecond\r\n",
        "é\n漢字\n".as_bytes(),
        b"\xff\x00\n\xfe\n",
    ] {
        let input = TestFile::new(bytes);
        assert_records(&sample(&input, 100, true, 42), 100, &records(bytes));
    }
}

#[test]
fn the_same_seed_repeats_the_same_sample() {
    let input = TestFile::new(b"aa\nbb\ncc\ndd\nee\n");
    for allow_duplicates in [false, true] {
        let count = if allow_duplicates { 100 } else { 3 };
        assert_eq!(
            sample(&input, count, allow_duplicates, 42),
            sample(&input, count, allow_duplicates, 42)
        );
    }
}

#[test]
fn different_seeds_can_select_different_samples() {
    let input = TestFile::new(b"aa\nbb\ncc\ndd\nee\n");
    assert_ne!(sample(&input, 100, true, 42), sample(&input, 100, true, 43));
}

fn assert_equal_chances(bytes: &[u8], expected: &[&[u8]]) {
    let input = TestFile::new(bytes);
    let count = 60_000;
    let output = sample(&input, count, true, 42);
    assert_records(&output, count, expected);
    let target = count / expected.len();
    let tolerance = target / 10;
    let counts: Vec<_> = expected
        .iter()
        .map(|line| {
            records(&output)
                .iter()
                .filter(|record| *record == line)
                .count()
        })
        .collect();
    assert!(
        counts
            .iter()
            .all(|&actual| actual.abs_diff(target) <= tolerance),
        "counts {counts:?}; target {target}; tolerance {tolerance}"
    );
}

#[test]
fn equal_length_lines_have_equal_selection_chances_including_the_first_line() {
    assert_equal_chances(
        b"aaaa\nbbbb\ncccc\ndddd\n",
        &[b"aaaa\n", b"bbbb\n", b"cccc\n", b"dddd\n"],
    );
}

#[test]
fn unequal_length_lines_have_equal_selection_chances() {
    assert_equal_chances(
        b"a\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\ncc\n",
        &[b"a\n", b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\n", b"cc\n"],
    );
}

#[test]
fn the_final_line_without_a_newline_has_the_same_selection_chance() {
    assert_equal_chances(b"aa\nbb\ncc", &[b"aa\n", b"bb\n", b"cc\n"]);
}

#[test]
fn a_unique_selection_also_has_equal_first_selection_chances() {
    let input = TestFile::new(b"a\nbbb\ncccccccc\ndddddddddddddddd\n");
    let expected = records(b"a\nbbb\ncccccccc\ndddddddddddddddd\n");
    let mut counts = vec![0usize; expected.len()];
    for seed in 0..4096 {
        let output = sample(&input, 1, false, seed);
        assert_records(&output, 1, &expected);
        let index = expected.iter().position(|&line| line == output).unwrap();
        counts[index] += 1;
    }
    assert!(
        counts.iter().all(|&actual| actual.abs_diff(1024) <= 200),
        "counts {counts:?}; target 1024; tolerance 200"
    );
}

#[test]
fn unique_two_line_subsets_have_equal_selection_chances() {
    let bytes = b"a\nbbb\ncccccccc\ndddddddddddddddd\n";
    let input = TestFile::new(bytes);
    let expected = records(bytes);
    let mut counts = [[0usize; 4]; 4];
    for seed in 0..8192 {
        let output = sample(&input, 2, false, seed);
        assert_records(&output, 2, &expected);
        let mut selected: Vec<_> = records(&output)
            .iter()
            .map(|line| {
                expected
                    .iter()
                    .position(|expected| expected == line)
                    .unwrap()
            })
            .collect();
        selected.sort_unstable();
        assert_ne!(selected[0], selected[1]);
        counts[selected[0]][selected[1]] += 1;
    }
    let target = 8192usize / 6;
    for (first, row) in counts.iter().enumerate() {
        for (second, &count) in row.iter().enumerate().skip(first + 1) {
            assert!(
                count.abs_diff(target) <= target / 5,
                "pair ({first}, {second}): count {count}; target {target}"
            );
        }
    }
}

struct FailingWriter;

impl Write for FailingWriter {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "test output failure",
        ))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn the_library_returns_output_errors() {
    let input = TestFile::new(b"one\ntwo\n");
    let error = quicklines(input.path(), 1, true, Some(42), &mut FailingWriter).unwrap_err();
    assert_eq!(
        error.downcast_ref::<io::Error>().unwrap().kind(),
        io::ErrorKind::BrokenPipe
    );
}

#[derive(Default)]
struct ShortWriter(Vec<u8>);

impl Write for ShortWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let count = bytes.len().min(1);
        self.0.extend_from_slice(&bytes[..count]);
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn short_writes_do_not_cut_output_lines() {
    let input = TestFile::new(b"first\nsecond\n");
    let mut writer = ShortWriter::default();
    quicklines(input.path(), 20, true, Some(42), &mut writer).unwrap();
    assert_records(&writer.0, 20, &[b"first\n", b"second\n"]);
}
