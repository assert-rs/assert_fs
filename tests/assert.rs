use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn code_example() {
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("foo.txt");
    input_file.touch().unwrap();

    // ... do something with input_file ...

    input_file.assert("");
    temp.child("bar.txt").assert(predicate::path::missing());

    temp.close().unwrap();
}

#[test]
#[should_panic]
fn verify_failure_output() {
    let f = assert_fs::fixture::ChildPath::new("Cargo.toml");
    f.assert("Not real content");
}
