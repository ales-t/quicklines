mod support;

use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use support::{assert_records, TestFile};

fn command(input: &TestFile, count: usize) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_quicklines"));
    command.args(["-c", &count.to_string(), "--seed", "42", input.path()]);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    command
}

// These commands produce small outputs that fit in the pipe.
fn bounded_output(mut command: Command) -> Output {
    let mut child = command.spawn().unwrap();
    let started = Instant::now();
    loop {
        if child.try_wait().unwrap().is_some() {
            return child.wait_with_output().unwrap();
        }
        if started.elapsed() > Duration::from_secs(5) {
            let _ = child.kill();
            let _ = child.wait();
            panic!("command did not finish within five seconds");
        }
        thread::sleep(Duration::from_millis(2));
    }
}

#[test]
fn the_cli_writes_the_requested_number_of_complete_lines() {
    let input = TestFile::new(b"aa\nbb\ncc\n");
    let output = bounded_output(command(&input, 20));
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_records(&output.stdout, 20, &[b"aa\n", b"bb\n", b"cc\n"]);
}

#[test]
fn the_cli_separates_repeated_lines_without_an_input_newline() {
    let input = TestFile::new(b"one");
    let output = bounded_output(command(&input, 3));
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"one\none\none\n");
}

#[test]
fn the_cli_rejects_a_huge_unique_request_without_partial_output() {
    let input = TestFile::new(b"one\ntwo\n");
    let mut command = command(&input, usize::MAX);
    command.arg("--no-duplicates");
    let output = bounded_output(command);
    assert!(!output.status.success());
    assert!(!output.stderr.is_empty());
    assert!(
        output.stdout.is_empty(),
        "check the line count before writing output"
    );
}

#[test]
fn the_cli_returns_an_error_for_a_missing_input_file() {
    let input = TestFile::new(b"");
    let path = format!("{}.missing", input.path());
    let mut command = Command::new(env!("CARGO_BIN_EXE_quicklines"));
    command.args(["-c", "1", &path]);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let output = bounded_output(command);
    assert!(!output.status.success());
    assert!(!output.stderr.is_empty());
    assert!(output.stdout.is_empty());
}

#[cfg(target_os = "linux")]
#[test]
fn the_cli_reports_a_final_buffered_output_error() {
    let input = TestFile::new(b"one\ntwo\n");
    let mut command = command(&input, 1);
    let full = std::fs::OpenOptions::new()
        .write(true)
        .open("/dev/full")
        .unwrap();
    command.stdout(full);
    let output = bounded_output(command);
    assert!(
        !output.status.success(),
        "the final output flush must report its error"
    );
    assert!(!output.stderr.is_empty());
}
