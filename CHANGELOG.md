<a name="0.3.0"></a>
## 0.3.0 (2018-06-27)


#### Features

* **assert:**
  *  Support `assert(bytes)` shorthand ([118004e4](https://github.com/assert-rs/assert_fs/commit/118004e48b5714613c0ddfec9022632be796c13c))
  *  Support `assert(str)` shorthand ([bf999396](https://github.com/assert-rs/assert_fs/commit/bf999396963c24dabcf01090b202d53f7fe82015))
* **fixture:**
  * copy_from now uses gitignore globs ([00d0f70b](https://github.com/assert-rs/assert_fs/commit/00d0f70be8ce303a38a6d74f528ff0868884816e))
  * Improve fixture error reporting ([89fe9ebb](https://github.com/assert-rs/assert_fs/commit/89fe9ebb5984cef90cc615701d36a6845c5445b8))

#### Bug Fixes

* **fixture:**
  * `copy_from(".")` failed ([3b2fcfd8](https://github.com/assert-rs/assert_fs/commit/3b2fcfd83dffb191e3f4a848aadcd3bb9499f038))

#### Breaking Changes

*   Rename traits to clarify intent ([f22f8eb1](https://github.com/assert-rs/assert_fs/commit/f22f8eb18a33ce504472bfce8b19b4cc29f5019b))



<a name="0.2.1"></a>
## 0.2.1 (2018-06-13)


* Documentation updates


<a name="0.2.0"></a>
## 0.2.0 (2018-05-30)

#### Bug Fixes

* **fixtures:**  copy_from now works ([f82317bb](https://github.com/assert-rs/assert_fs/commit/f82317bb97ecfedd0821ae0d88bb254412080976))

#### Features

*   Filesystem assertions ([3ba02a93](https://github.com/assert-rs/assert_fs/commit/3ba02a9343101447ac90dca5eeeb6353c25ad646), closes [#7](https://github.com/assert-rs/assert_fs/issues/7))

#### Breaking Changes

* Update version of `predicates-rs` to v0.5.0.

<a name="0.1.1"></a>
## 0.1.1 (2018-05-28)


#### Features

*   Add a prelude ([43539abf](https://github.com/assert-rs/assert_fs/commit/43539abff3e3ee879b763f5049817ca7d8609fed))



