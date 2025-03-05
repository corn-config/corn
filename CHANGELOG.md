# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.10.0] - 2024-08-23

### :sparkles: New Features

- [`2273ce7`](https://github.com/corn-config/corn/commit/2273ce77b819895022d62dec8a8446f9e0345260) -
  trim whitespace from multi-line strings _(commit by @JakeStanger)_
- [`91b8d5c`](https://github.com/corn-config/corn/commit/91b8d5c976bd940cc59d8df250101ff9a544a8fe) -
  ability to quote keys to escape limitations

## [v0.9.2] - 2023-09-14

### :bug: Bug Fixes

- [`d124160`](https://github.com/JakeStanger/corn/commit/d124160ca425cd071ab269a5d2baf48035476798) -
  outputted keys not in same order as input _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :memo: Documentation Changes

- [`f39d24f`](https://github.com/JakeStanger/corn/commit/f39d24f502c7d852f482669faf78ac7fac591a91) -
  update CHANGELOG.md for v0.9.1 [skip ci] _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

## [v0.9.1] - 2023-09-01

### :bug: Bug Fixes

- [`f49f68b`](https://github.com/JakeStanger/corn/commit/f49f68b7cb3ed16a912cd45305d22c10d4e05f57) -
  unable to escape input in string interpolation _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :memo: Documentation Changes

- [`d848156`](https://github.com/JakeStanger/corn/commit/d848156e9f03e034322dfc0e49bed662e802eb52) -
  update CHANGELOG.md for v0.9.0 [skip ci] _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`6ae953e`](https://github.com/JakeStanger/corn/commit/6ae953e1e3d3e464b00b794390daef05cdcdfd1c) -
  **readme**: fix link to rust docs _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

## [v0.9.0] - 2023-08-30

### :sparkles: New Features

- [`7b415b2`](https://github.com/JakeStanger/corn/commit/7b415b2deac4c5e1315a46940025c461f7fbc7bc) -
  **cli**: stdin support _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`848f706`](https://github.com/JakeStanger/corn/commit/848f70655a9a6ca6513e7d6e6dead1033ff1d7f6) -
  **libcorn**: lua support via library export _(commit by
  [@A-Cloud-Ninja](https://github.com/A-Cloud-Ninja))_

### :bug: Bug Fixes

- [`b0bc2fe`](https://github.com/JakeStanger/corn/commit/b0bc2fed7fea47ff76dec0067e2f1494044a1fb6) -
  useful info ommitted from errors _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :memo: Documentation Changes

- [`35a4a83`](https://github.com/JakeStanger/corn/commit/35a4a8399f01bfec73bf3578e8d56d4b328bc0d8) -
  update CHANGELOG.md for v0.8.0 [skip ci] _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`1a778ab`](https://github.com/JakeStanger/corn/commit/1a778ab350a27d4aa82ea2d8292066b244edcdef) -
  **readme**: fix link to tree-sitter parser _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`35c60e8`](https://github.com/JakeStanger/corn/commit/35c60e83f13bdf6b395b597c3f4b1e9436bca645) -
  add panic info for parser _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

## [v0.8.0] - 2023-06-18

### :sparkles: New Features

- [`301ceea`](https://github.com/JakeStanger/corn/commit/301ceea0c4dafffd38f4688fcf1df6d7bfdb448b) -
  **parser**: add hex and underscore separator support to integers. _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`53afac7`](https://github.com/JakeStanger/corn/commit/53afac74dce229f57873e1af3edd6e86cb793ce7) -
  **parser**: add escape char, interpolation support to strings _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :bug: Bug Fixes

- [`d51bf5a`](https://github.com/JakeStanger/corn/commit/d51bf5abd4a6ad2b86cb8b1e8a5fd3a65ac0ea30) -
  **parser**: crash when spreading invalid type _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :recycle: Refactors

- [`5d2b7c8`](https://github.com/JakeStanger/corn/commit/5d2b7c85ecb6431a1f2cebf39366024224e389c1) -
  remove no longer required `TomlValue` struct _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`dcffab6`](https://github.com/JakeStanger/corn/commit/dcffab62803d3b4c8ccb91da219781cf673765fe) -
  **cli**: reduce duplicate code _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`047d1d3`](https://github.com/JakeStanger/corn/commit/047d1d3f9cc037ccf3827ccc264b50e5e0536662) -
  **parser**: minor env var performance improvement _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`42fa830`](https://github.com/JakeStanger/corn/commit/42fa830d0bd2f9fefc86b8e841c3abbbd0fe68f2) -
  fix clippy warning _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :white_check_mark: Tests

- [`c7253db`](https://github.com/JakeStanger/corn/commit/c7253dbef9782c8d85cd1b285112532da653207e) -
  fix invalid spread test input _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`0ec37d8`](https://github.com/JakeStanger/corn/commit/0ec37d871742ab46c5c3b3ad3732c0444413f839) -
  add benchmarking _(commit by [@JakeStanger](https://github.com/JakeStanger))_
- [`693c91e`](https://github.com/JakeStanger/corn/commit/693c91ec50aca1004b79b74b30313cd798d3cfac) -
  add coverage for float exponent syntax _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`de57c71`](https://github.com/JakeStanger/corn/commit/de57c71b9713307ce0dbed47c4d7572bf71eb116) -
  **invalid spread**: add case for array spread _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :memo: Documentation Changes

- [`e760b8c`](https://github.com/JakeStanger/corn/commit/e760b8ceaf2428c691ff6a8abfc6e92cd610c02e) -
  update CHANGELOG.md for v0.7.0 [skip ci] _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`c4cd6a8`](https://github.com/JakeStanger/corn/commit/c4cd6a8218c09480c22f151c01e4f8b888c6fa7b) -
  **readme**: add nvim section _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`6f5f066`](https://github.com/JakeStanger/corn/commit/6f5f06683a448c765bb64cc59a9fc08b16e20762) -
  **readme**: update to cover new features _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`749fe37`](https://github.com/JakeStanger/corn/commit/749fe379adc4c02865f756ef8d29641a0e8ba185) -
  add landing page readmes for crates _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

## [v0.7.0] - 2023-05-24

### :sparkles: New Features

- [`48304d4`](https://github.com/JakeStanger/corn/commit/48304d4d809c1bcb3fdedfdffe3377952ca2a767) -
  spread operator _(commit by [@JakeStanger](https://github.com/JakeStanger))_

### :recycle: Refactors

- [`ab1af29`](https://github.com/JakeStanger/corn/commit/ab1af29219dc82ce86e32eb466a9d24ee6f195b6) -
  improve error handling and code quality _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :memo: Documentation Changes

- [`2be933d`](https://github.com/JakeStanger/corn/commit/2be933dc949b4357b9938643f3abe3ff22f33e39) -
  update CHANGELOG.md for v0.6.1 [skip ci] _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`17a99d2`](https://github.com/JakeStanger/corn/commit/17a99d2d0939b4fb0e197b2ad061fe13dfdb5bb2) -
  **readme**: correct a few bits, add more detail _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

## [v0.6.1] - 2022-12-12

### :bug: Bug Fixes

- [`b6e93b2`](https://github.com/JakeStanger/corn/commit/b6e93b202d961f51ce6c92c58a9ed30111a820af) -
  **lib**: deserializer not handling invalid inputs _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :recycle: Refactors

- [`21e1ee0`](https://github.com/JakeStanger/corn/commit/21e1ee03cb3e81ea1e8dd97fd300fbb12fcb8341) -
  tidy error handling _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :memo: Documentation Changes

- [`9dbb9d6`](https://github.com/JakeStanger/corn/commit/9dbb9d6dc3dc018f47f38b270d31cadc2406d8be) -
  update CHANGELOG.md for v0.6.0 [skip ci] _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

## [v0.6.0] - 2022-11-28

### :sparkles: New Features

- [`7a2f7b5`](https://github.com/JakeStanger/corn/commit/7a2f7b5a961689413ccc8f9b1fb75f998ceebac8) -
  **de**: `from_slice` func _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :bug: Bug Fixes

- [`e6c8e90`](https://github.com/JakeStanger/corn/commit/e6c8e901ac87d01137cd06e4317cf009e7325e59) -
  **de**: from_str panicking instead of returning result _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_
- [`7ea024d`](https://github.com/JakeStanger/corn/commit/7ea024d047862b89c57b78cb8480009514221d24) -
  **parser**: panic when input references another input _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

## [v0.5.0] - 2022-11-27

### :sparkles: New Features

- [`9fbf1b0`](https://github.com/JakeStanger/corn/commit/9fbf1b0c9ca53c14f787a997bbb067d918142b24) -
  serde deserialization support _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

### :white_check_mark: Tests

- [`d035fa2`](https://github.com/JakeStanger/corn/commit/d035fa2fd92a5e62081b7d51a56d63222bb6e73e) -
  update test assets _(commit by
  [@JakeStanger](https://github.com/JakeStanger))_

[v0.5.0]: https://github.com/JakeStanger/corn/compare/v0.4.0...v0.5.0
[v0.6.0]: https://github.com/JakeStanger/corn/compare/v0.5.0...v0.6.0
[v0.6.1]: https://github.com/JakeStanger/corn/compare/v0.6.0...v0.6.1
[v0.7.0]: https://github.com/JakeStanger/corn/compare/v0.6.1...v0.7.0
[v0.8.0]: https://github.com/JakeStanger/corn/compare/v0.7.0...v0.8.0
[v0.9.0]: https://github.com/JakeStanger/corn/compare/v0.8.0...v0.9.0
[v0.9.1]: https://github.com/JakeStanger/corn/compare/v0.9.0...v0.9.1
[v0.9.2]: https://github.com/JakeStanger/corn/compare/v0.9.1...v0.9.2
[v0.10.0]: https://github.com/corn-config/corn/compare/v0.9.2...v0.10.0
