use quicklines::quicklines;

use std::fs;
use std::io::Cursor;

fn test_resources_dir() -> String {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/resources/").to_owned()
}

#[test]
#[ignore]
fn get_all_uniformly_spaced_lines() {
    // Given an input file with 5 lines of identical lengths
    // let input_path =
    //     String::from_iter([test_resources_dir(), "input-uniform-lines.txt".to_owned()]);
    // let mut writer = Cursor::new(vec![]);

    // // When we request 5 lines
    // quicklines(&input_path, 5, &mut writer).unwrap();

    // // We should get back exactly the original file
    // assert_eq!(
    //     &writer.get_ref()[..],
    //     fs::read_to_string(&input_path).unwrap().as_bytes()
    // );
}

#[test]
#[ignore]
fn step_is_nonzero() {
    // Given a small input file
    // let input_path =
    //     String::from_iter([test_resources_dir(), "input-uniform-lines.txt".to_owned()]);
    // let mut writer = Cursor::new(vec![]);

    // // When we request far too many lines
    // quicklines(&input_path, 1_000_000, &mut writer).unwrap();

    // // We should get back at most (in this case exactly) the original file
    // assert_eq!(
    //     &writer.get_ref()[..],
    //     fs::read_to_string(&input_path).unwrap().as_bytes()
    // );
}
