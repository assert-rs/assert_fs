#![allow(clippy::unwrap_used)]

#[allow(clippy::wildcard_imports)] // false positive
use assert_fs::prelude::*;

fn main() {
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("foo.txt");

    input_file.write_str("Hello\nWorld!").unwrap();

    input_file.assert("Goodbye\nWorld!");

    temp.close().unwrap();
}
