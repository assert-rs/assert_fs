# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [1.1.0] - 2023-12-27

### Compatibility

- Update MSRV to 1.70.0

### Documentation

- Update a stale reference

## [1.0.13] - 2023-04-13

### Internal

- Dependency update

## [1.0.12] - 2023-03-16

### Internal

- Dependency update

## [1.0.11] - 2023-03-14

### Fixes

- Correctly handle `CLICOLOR=1`
- Correctly handle `NO_COLOR=`
- Auto-enable color on CI

## [1.0.10] - 2022-12-02

## [1.0.9] - 2022-11-07

### Fixes

- Add `Debug` impls for temp dir / file

## [1.0.8] - 2022-11-04

### Fixes

- Hide internal-only optional dependencies

## [1.0.7] - 2022-01-11

## [1.0.6] - 2021-10-07

## [1.0.5] - 2021-09-06

#### Fixes

- Show caller, rather than `assert_fs`, as cause of panics

## [1.0.4] - 2021-08-30

#### Features

- File and directory symlinks

## [1.0.3] - 2021-07-03

## [1.0.2] - 2021-04-22

#### Fixes

* Explicitly stated that `assert` may `panic`

#### Features

* Built-in fixtures now interoperate as `&Path`

## [1.0.1] - 2021-02-01

## 1.0.0 - 2020-03-26

Stable release!

## 0.13.1 - 2019-12-01


#### Features

* **assert:**  Support more string types in shortcut



## 0.13.0 - 2019-11-29


#### Breaking Changes

* **persist:**  Clarify API behavior
* `copy_from` now expects globs relative to the root (so change `*` to `**`)

#### Bug Fixes

* **persist:**  Clarify API behavior
* `copy_from` now expects globs relative to the root (so change `*` to `**`)



## 0.11.3 - 2019-01-29


#### Features

* **fixture:**  Another subdir route



## 0.11.2 - 2019-01-29


#### Features

* **assert:**  Support NamedTempFile



## 0.11.1 - 2019-01-29


#### Features

* **fixture:**  Shorten route to Temp File



## 0.11.0 - 2019-01-29


#### Breaking Changes

* **fixture:**
  *  Unify on error type
  *  Newtype for TempDir

#### Features

* **fixture:**
  *  Debug persistence support
  *  Auto-create directories
  *  NamedTempFile support
  *  Copy a file
  *  Support creating dirs



## 0.10.1 - 2019-01-07


#### Bug Fixes

* **assert:**  Show why assert failed



## 0.10.0 - 2018-10-26


#### Breaking Changes

*   Re-structure API

#### Bug Fixes

*   Expose errors where relevant in the API
*   Re-structure API

#### Features

* **assert:**  Accept Predicate<str>



## 0.9.0 - 2018-08-02


#### Breaking Changes

*   Bury errors in the docs
*   Remove failure from API
*   Rename traits to clarify intent

#### Features

* **assert:**
  *  Show cause of failure
  *  Support assert(bytes) shorthand
  *  Use DifferencePredicate for str
  *  Predicate<[u8]> acts on file contents

#### Bug Fixes

*   Bury errors in the docs
*   Remove failure from API
*   Rename traits to clarify intent
* **assert:**
  *  Isolate API details



## 0.3.0 - 2018-06-27


#### Features

* **assert:**
  *  Support `assert(bytes)` shorthand
  *  Support `assert(str)` shorthand
* **fixture:**
  * copy_from now uses gitignore globs
  * Improve fixture error reporting

#### Bug Fixes

* **fixture:**
  * `copy_from(".")` failed

#### Breaking Changes

*   Rename traits to clarify intent



## 0.2.1 - 2018-06-13


* Documentation updates


## 0.2.0 - 2018-05-30

#### Bug Fixes

* **fixtures:**  copy_from now works

#### Features

*   Filesystem assertions

#### Breaking Changes

* Update version of `predicates-rs` to v0.5.0.

## 0.1.1 - 2018-05-28


#### Features

*   Add a prelude


<!-- next-url -->
[Unreleased]: https://github.com/assert-rs/assert_fs/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/assert-rs/assert_fs/compare/v1.0.13...v1.1.0
[1.0.13]: https://github.com/assert-rs/assert_fs/compare/v1.0.12...v1.0.13
[1.0.12]: https://github.com/assert-rs/assert_fs/compare/v1.0.11...v1.0.12
[1.0.11]: https://github.com/assert-rs/assert_fs/compare/v1.0.10...v1.0.11
[1.0.10]: https://github.com/assert-rs/assert_fs/compare/v1.0.9...v1.0.10
[1.0.9]: https://github.com/assert-rs/assert_fs/compare/v1.0.8...v1.0.9
[1.0.8]: https://github.com/assert-rs/assert_fs/compare/v1.0.7...v1.0.8
[1.0.7]: https://github.com/assert-rs/assert_fs/compare/v1.0.6...v1.0.7
[1.0.6]: https://github.com/assert-rs/assert_fs/compare/v1.0.5...v1.0.6
[1.0.5]: https://github.com/assert-rs/assert_fs/compare/v1.0.4...v1.0.5
[1.0.4]: https://github.com/assert-rs/assert_fs/compare/v1.0.3...v1.0.4
[1.0.3]: https://github.com/assert-rs/assert_fs/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/assert-rs/assert_fs/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/assert-rs/assert_fs/compare/v1.0.0...v1.0.1
