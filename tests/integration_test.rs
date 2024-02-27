use quicklines::quicklines;

use std::collections::HashSet;
use std::io::{BufRead, Cursor};

fn test_resources_dir() -> String {
    String::from_iter([env!("CARGO_MANIFEST_DIR"), "/tests/resources/"])
}

#[test]
fn sampling_with_replacement_contains_duplicates() {
    // Given an input file
    let input_path =
        String::from_iter([test_resources_dir(), "input-uniform-lines.txt".to_owned()]);
    let mut writer = Cursor::new(vec![]);

    // When we request a subset of lines with a well-chosen seed
    quicklines(&input_path, 3, true, Some(12), &mut writer).unwrap();

    // Then the total number of lines should be as requested
    let lines: Vec<String> = writer.get_ref().lines().map(|line| line.unwrap()).collect();
    assert_eq!(lines.len(), 3);

    // Then the number of unique lines may be lower
    let lines_set: HashSet<_> = lines.into_iter().collect();
    assert_eq!(lines_set.len(), 2);
}

#[test]
fn sampling_with_replacement_allows_oversampling() {
    // Given an input file
    let input_path =
        String::from_iter([test_resources_dir(), "input-uniform-lines.txt".to_owned()]);
    let mut writer = Cursor::new(vec![]);

    // When we request a superset of lines
    quicklines(&input_path, 100, true, Some(12), &mut writer).unwrap();

    // Then the total number of lines should be as requested
    let lines: Vec<String> = writer.get_ref().lines().map(|line| line.unwrap()).collect();
    assert_eq!(lines.len(), 100);

    // Then the number of unique lines is at most the number of lines in the file
    let lines_set: HashSet<_> = lines.into_iter().collect();
    assert_eq!(lines_set.len(), 5);
}

#[test]
fn sampling_without_replacement_does_not_contain_duplicates() {
    // Given an input file
    let input_path =
        String::from_iter([test_resources_dir(), "input-uniform-lines.txt".to_owned()]);
    let mut writer = Cursor::new(vec![]);

    // When we request all of the lines without replacement
    quicklines(&input_path, 5, false, Some(12), &mut writer).unwrap();

    // Then the total number of lines should be as requested
    let lines: Vec<String> = writer.get_ref().lines().map(|line| line.unwrap()).collect();
    assert_eq!(lines.len(), 5);

    // Then the number of unique lines should be equal to the total
    let lines_set: HashSet<_> = lines.iter().collect();
    assert_eq!(lines_set.len(), lines.len());
}
