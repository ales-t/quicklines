mod support;

use std::fs::File;
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};

use memchr::memchr_iter;
use support::TestFile;

fn uniform_file(lines: usize, line_bytes: usize) -> TestFile {
    TestFile::generate(|writer| {
        let mut line = vec![b'x'; line_bytes];
        line[line_bytes - 1] = b'\n';
        for id in 0..lines {
            line[..8].copy_from_slice(format!("{id:08}").as_bytes());
            writer.write_all(&line)?;
        }
        Ok(())
    })
}

fn count_lines(input: &TestFile) -> usize {
    let mut file = File::open(input.path()).unwrap();
    let mut buffer = vec![0; 1024 * 1024];
    let mut count = 0;
    loop {
        let bytes = file.read(&mut buffer).unwrap();
        if bytes == 0 {
            return count;
        }
        count += memchr_iter(b'\n', &buffer[..bytes]).count();
    }
}

fn median_time(mut operation: impl FnMut()) -> Duration {
    operation();
    let mut times = Vec::new();
    for _ in 0..3 {
        let start = Instant::now();
        operation();
        times.push(start.elapsed());
    }
    times.sort_unstable();
    times[1]
}

#[derive(Default)]
struct CountingWriter {
    bytes: usize,
}

impl Write for CountingWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.bytes += bytes.len();
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn selection_time(
    input: &TestFile,
    count: usize,
    line_bytes: usize,
    allow_duplicates: bool,
) -> Duration {
    median_time(|| {
        let mut output = CountingWriter::default();
        quicklines::quicklines(input.path(), count, allow_duplicates, Some(42), &mut output)
            .expect("selection must succeed");
        assert_eq!(output.bytes, count * line_bytes, "wrong output size");
    })
}

fn scan_time(input: &TestFile, lines: usize) -> Duration {
    median_time(|| assert_eq!(count_lines(input), lines))
}

fn require_release_build() {
    assert!(
        !cfg!(debug_assertions),
        "run speed tests with cargo test --release"
    );
}

#[test]
#[ignore = "speed test; use a release build and one test thread"]
fn a_small_sample_costs_less_than_a_full_scan() {
    require_release_build();
    let lines = 2_097_152;
    let input = uniform_file(lines, 128);
    let scan = scan_time(&input, lines);
    let selection = selection_time(&input, 32, 128, true);
    let budget = scan.mul_f64(0.5) + Duration::from_millis(2);
    eprintln!("small sample: selection={selection:?}; scan={scan:?}; budget={budget:?}");
    assert!(
        selection <= budget,
        "a small sample must not pay for a full scan"
    );
}

#[test]
#[ignore = "speed test; use a release build and one test thread"]
fn a_complete_unique_sample_stays_within_a_full_scan_budget() {
    require_release_build();
    let lines = 1_048_576;
    let input = uniform_file(lines, 128);
    let scan = scan_time(&input, lines);
    let selection = selection_time(&input, lines, 128, false);
    let budget = scan * 20 + Duration::from_millis(100);
    eprintln!("complete sample: selection={selection:?}; scan={scan:?}; budget={budget:?}");
    assert!(
        selection <= budget,
        "a complete sample must not use too many repeated attempts"
    );
}

#[test]
#[ignore = "speed test; use a release build and one test thread"]
fn long_line_selection_stays_within_a_full_scan_budget() {
    require_release_build();
    let lines = 512;
    let line_bytes = 256 * 1024;
    let input = uniform_file(lines, line_bytes);
    let scan = scan_time(&input, lines);
    let selection = selection_time(&input, 512, line_bytes, true);
    let budget = scan * 20 + Duration::from_millis(100);
    eprintln!("long lines: selection={selection:?}; scan={scan:?}; budget={budget:?}");
    assert!(
        selection <= budget,
        "long lines must not cause too many random attempts"
    );
}
