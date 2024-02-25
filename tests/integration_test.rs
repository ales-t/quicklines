use quicklines::quicklines;

use std::fs;
use std::io::Cursor;

fn test_resources_dir() -> String {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/resources/").to_owned()
}

#[test]
fn uniform_get_all_lines() {
    let input_path =
        String::from_iter([test_resources_dir(), "input-uniform-lines.txt".to_owned()]);
    let mut writer = Cursor::new(vec![]);

    quicklines(&input_path, 5, &mut writer).unwrap();

    assert_eq!(
        &writer.get_ref()[..],
        fs::read_to_string(&input_path).unwrap().as_bytes()
    );
}
