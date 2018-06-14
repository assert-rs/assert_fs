extern crate assert_fs;
extern crate predicates;

use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn code_example() {
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("foo.txt");
    input_file.touch().unwrap();
    // ... do something with input_file ...
    input_file.assert(predicate::str::is_empty().from_utf8());
    temp.child("bar.txt").assert(predicate::path::missing());
    temp.close().unwrap();
}
