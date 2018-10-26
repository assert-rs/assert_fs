# assert_fs

> **Assert Filesystems** - Filesystem fixtures and assertions for testing.

[![Travis Status](https://travis-ci.org/assert-rs/assert_fs.svg?branch=master)](https://travis-ci.org/assert-rs/assert_fs)
[![Appveyor Status](https://ci.appveyor.com/api/projects/status/kyjb91uim1xpyi8y/branch/master?svg=true)](https://ci.appveyor.com/project/epage/assert-fs/branch/master)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/assert_fs.svg)
[![Crates Status](https://img.shields.io/crates/v/assert_fs.svg)](https://crates.io/crates/assert_fs)

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
assert_fs = "0.10"
```

## Example

Here is a trivial example:

```rust,ignore
use assert_fs::prelude::*;
use predicates::prelude::*;

let temp = assert_fs::TempDir::new().unwrap();
let input_file = temp.child("foo.txt");
input_file.touch().unwrap();
// ... do something with input_file ...
input_file.assert("");
temp.child("bar.txt").assert(predicate::path::missing());
temp.close().unwrap();
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[Crates.io]: https://crates.io/crates/assert_fs
[Documentation]: https://docs.rs/assert_fs
