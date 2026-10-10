// Each test file uses a different set of these helpers.
#![allow(dead_code)]

use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_FILE: AtomicUsize = AtomicUsize::new(0);

pub struct TestFile {
    directory: PathBuf,
    path: PathBuf,
}

impl TestFile {
    pub fn new(bytes: &[u8]) -> Self {
        Self::generate(|writer| writer.write_all(bytes))
    }

    pub fn generate(write: impl FnOnce(&mut BufWriter<File>) -> io::Result<()>) -> Self {
        let root = PathBuf::from("/tmp/opencode");
        fs::create_dir_all(&root).unwrap();
        let directory = loop {
            let id = NEXT_FILE.fetch_add(1, Ordering::Relaxed);
            let directory = root.join(format!("quicklines-test-{}-{id}", std::process::id()));
            match fs::create_dir(&directory) {
                Ok(()) => break directory,
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("cannot create test directory: {error}"),
            }
        };
        let input = Self {
            path: directory.join("input.txt"),
            directory,
        };
        let mut writer = BufWriter::with_capacity(1024 * 1024, File::create(&input.path).unwrap());
        write(&mut writer).unwrap();
        writer.flush().unwrap();
        input
    }

    pub fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
        let _ = fs::remove_dir(&self.directory);
    }
}

pub fn sample(input: &TestFile, count: usize, allow_duplicates: bool, seed: u64) -> Vec<u8> {
    let mut output = Vec::new();
    quicklines::quicklines(
        input.path(),
        count,
        allow_duplicates,
        Some(seed),
        &mut output,
    )
    .expect("selection must succeed");
    output
}

pub fn records(output: &[u8]) -> Vec<&[u8]> {
    output.split_inclusive(|&byte| byte == b'\n').collect()
}

pub fn assert_records(output: &[u8], count: usize, expected: &[&[u8]]) {
    let actual = records(output);
    assert_eq!(actual.len(), count, "wrong output line count");
    for record in actual {
        assert!(record.ends_with(b"\n"), "output line has no newline");
        assert!(
            expected.contains(&record),
            "output is not a complete input line"
        );
    }
}
