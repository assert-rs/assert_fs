<a name="0.13.1"></a>
## 1.0.0 (2020-03-26)

Stable release!

<a name="0.13.1"></a>
## 0.13.1 (2019-12-01)


#### Features

* **assert:**  Support more string types in shortcut ([3de84ce4](https://github.com/assert-rs/assert_fs/commit/3de84ce453e1e1024331421e0ff1071cab5f204b))



<a name="0.13.0"></a>
## 0.13.0 (2019-11-29)


#### Breaking Changes

* **persist:**  Clarify API behavior ([5411d07c](https://github.com/assert-rs/assert_fs/commit/5411d07c37fbfb89e3efde2c4562a6dd4d0f1ebb), closes [#51](https://github.com/assert-rs/assert_fs/issues/51), breaks [#](https://github.com/assert-rs/assert_fs/issues/))
* `copy_from` now expects globs relative to the root (so change `*` to `**`)

#### Bug Fixes

* **persist:**  Clarify API behavior ([5411d07c](https://github.com/assert-rs/assert_fs/commit/5411d07c37fbfb89e3efde2c4562a6dd4d0f1ebb), closes [#51](https://github.com/assert-rs/assert_fs/issues/51), breaks [#](https://github.com/assert-rs/assert_fs/issues/))
* `copy_from` now expects globs relative to the root (so change `*` to `**`)



<a name="0.11.3"></a>
## 0.11.3 (2019-01-29)


#### Features

* **fixture:**  Another subdir route ([0959d5f6](https://github.com/assert-rs/assert_fs/commit/0959d5f6f75b46f4539e52d651824dab9449b339))



<a name="0.11.2"></a>
## 0.11.2 (2019-01-29)


#### Features

* **assert:**  Support NamedTempFile ([935b57dd](https://github.com/assert-rs/assert_fs/commit/935b57dd5d67e82db2712fefd7020c427724a77e))



<a name="0.11.1"></a>
## 0.11.1 (2019-01-29)


#### Features

* **fixture:**  Shorten route to Temp File ([73de421b](https://github.com/assert-rs/assert_fs/commit/73de421b8f446ade35e29e07fe2207a4a84c7200))



<a name="0.11.0"></a>
## 0.11.0 (2019-01-29)


#### Breaking Changes

* **fixture:**
  *  Unify on error type ([27566755](https://github.com/assert-rs/assert_fs/commit/275667551b78d7e2ad702f81150417a782b64253), closes [#39](https://github.com/assert-rs/assert_fs/issues/39), breaks [#](https://github.com/assert-rs/assert_fs/issues/))
  *  Newtype for TempDir ([39b2913f](https://github.com/assert-rs/assert_fs/commit/39b2913f9e9e0a7ccc55b788fe04d5cb55264244), breaks [#](https://github.com/assert-rs/assert_fs/issues/))

#### Features

* **fixture:**
  *  Debug persistence support ([82220c82](https://github.com/assert-rs/assert_fs/commit/82220c8270fed8f021e6fb76f89864be0d8eda88), closes [#31](https://github.com/assert-rs/assert_fs/issues/31))
  *  Auto-create directories ([c6f8332c](https://github.com/assert-rs/assert_fs/commit/c6f8332c1a106c89f9df166879051f76f9090db5))
  *  NamedTempFile support ([e5209dfc](https://github.com/assert-rs/assert_fs/commit/e5209dfc6e2f2515cff2f0ea0189537c9fb117b3), closes [#33](https://github.com/assert-rs/assert_fs/issues/33))
  *  Copy a file ([4209ad29](https://github.com/assert-rs/assert_fs/commit/4209ad29862a02173e181f1041152e9fcb065aba), closes [#34](https://github.com/assert-rs/assert_fs/issues/34))
  *  Support creating dirs ([f7adecde](https://github.com/assert-rs/assert_fs/commit/f7adecde52d2fbb56a9542b0b693292ca9dd44e6), closes [#36](https://github.com/assert-rs/assert_fs/issues/36))



<a name="0.10.1"></a>
## 0.10.1 (2019-01-07)


#### Bug Fixes

* **assert:**  Show why assert failed ([8079b749](https://github.com/assert-rs/assert_fs/commit/8079b749cf1b1334e2dea42dc74f9552a0969cd7), closes [#37](https://github.com/assert-rs/assert_fs/issues/37))



<a name="0.10.0"></a>
## 0.10.0 (2018-10-26)


#### Breaking Changes

*   Re-structure API ([6c9876d7](https://github.com/assert-rs/assert_fs/commit/6c9876d76052d89af3edccbc66b073b085d9ecdb), breaks [#](https://github.com/assert-rs/assert_fs/issues/))

#### Bug Fixes

*   Expose errors where relevant in the API ([d04cd8f9](https://github.com/assert-rs/assert_fs/commit/d04cd8f975f13104e2fd0c7ad6b3cb82c2239701))
*   Re-structure API ([6c9876d7](https://github.com/assert-rs/assert_fs/commit/6c9876d76052d89af3edccbc66b073b085d9ecdb), breaks [#](https://github.com/assert-rs/assert_fs/issues/))

#### Features

* **assert:**  Accept Predicate<str> ([e7f174aa](https://github.com/assert-rs/assert_fs/commit/e7f174aae24a2e67e5195ffce5f91993e391589f), closes [#25](https://github.com/assert-rs/assert_fs/issues/25))



<a name="0.9.0"></a>
## 0.9.0 (2018-08-02)


#### Breaking Changes

*   Bury errors in the docs ([64f7b49d](https://github.com/assert-rs/assert_fs/commit/64f7b49d2036e132d9aa270db209d9b977e4ad3d))
*   Remove failure from API ([22146f11](https://github.com/assert-rs/assert_fs/commit/22146f113ff0b5da95c22058b12117ac4b712d73), closes [#14](https://github.com/assert-rs/assert_fs/issues/14))
*   Rename traits to clarify intent ([f22f8eb1](https://github.com/assert-rs/assert_fs/commit/f22f8eb18a33ce504472bfce8b19b4cc29f5019b))

#### Features

* **assert:**
  *  Show cause of failure ([befd9682](https://github.com/assert-rs/assert_fs/commit/befd9682776729cb5c05a5eea4f242711b6c3b85), closes [#3](https://github.com/assert-rs/assert_fs/issues/3))
  *  Support assert(bytes) shorthand ([118004e4](https://github.com/assert-rs/assert_fs/commit/118004e48b5714613c0ddfec9022632be796c13c))
  *  Use DifferencePredicate for str ([cfb49e57](https://github.com/assert-rs/assert_fs/commit/cfb49e578b54c89165932062378b24c872d1b5d8))
  *  Predicate<[u8]> acts on file contents ([43ec578f](https://github.com/assert-rs/assert_fs/commit/43ec578f0ebd9fac8229d84a23417566d83bac3e))

#### Bug Fixes

*   Bury errors in the docs ([64f7b49d](https://github.com/assert-rs/assert_fs/commit/64f7b49d2036e132d9aa270db209d9b977e4ad3d))
*   Remove failure from API ([22146f11](https://github.com/assert-rs/assert_fs/commit/22146f113ff0b5da95c22058b12117ac4b712d73), closes [#14](https://github.com/assert-rs/assert_fs/issues/14))
*   Rename traits to clarify intent ([f22f8eb1](https://github.com/assert-rs/assert_fs/commit/f22f8eb18a33ce504472bfce8b19b4cc29f5019b))
* **assert:**
  *  Isolate API details ([4f222646](https://github.com/assert-rs/assert_fs/commit/4f2226461bcc5b5f96957db0ebb8363cfa5f84d7))



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



