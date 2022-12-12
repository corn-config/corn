# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.6.1] - 2022-12-12
### :bug: Bug Fixes
- [`b6e93b2`](https://github.com/JakeStanger/corn/commit/b6e93b202d961f51ce6c92c58a9ed30111a820af) - **lib**: deserializer not handling invalid inputs *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :recycle: Refactors
- [`21e1ee0`](https://github.com/JakeStanger/corn/commit/21e1ee03cb3e81ea1e8dd97fd300fbb12fcb8341) - tidy error handling *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :memo: Documentation Changes
- [`9dbb9d6`](https://github.com/JakeStanger/corn/commit/9dbb9d6dc3dc018f47f38b270d31cadc2406d8be) - update CHANGELOG.md for v0.6.0 [skip ci] *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v0.6.0] - 2022-11-28
### :sparkles: New Features
- [`7a2f7b5`](https://github.com/JakeStanger/corn/commit/7a2f7b5a961689413ccc8f9b1fb75f998ceebac8) - **de**: `from_slice` func *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :bug: Bug Fixes
- [`e6c8e90`](https://github.com/JakeStanger/corn/commit/e6c8e901ac87d01137cd06e4317cf009e7325e59) - **de**: from_str panicking instead of returning result *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`7ea024d`](https://github.com/JakeStanger/corn/commit/7ea024d047862b89c57b78cb8480009514221d24) - **parser**: panic when input references another input *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v0.5.0] - 2022-11-27
### :sparkles: New Features
- [`9fbf1b0`](https://github.com/JakeStanger/corn/commit/9fbf1b0c9ca53c14f787a997bbb067d918142b24) - serde deserialization support *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :white_check_mark: Tests
- [`d035fa2`](https://github.com/JakeStanger/corn/commit/d035fa2fd92a5e62081b7d51a56d63222bb6e73e) - update test assets *(commit by [@JakeStanger](https://github.com/JakeStanger))*


[v0.5.0]: https://github.com/JakeStanger/corn/compare/v0.4.0...v0.5.0
[v0.6.0]: https://github.com/JakeStanger/corn/compare/v0.5.0...v0.6.0
[v0.6.1]: https://github.com/JakeStanger/corn/compare/v0.6.0...v0.6.1