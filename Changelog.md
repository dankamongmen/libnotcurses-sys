# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]
- update vendored bindings.
- fix `NcVisual::from_sixel`.
- make `NcFile::from_nc` unsafe.
- add `eff_text` field to `NcInput`.

## [3.10.1] - 2024-02-22
- update `libc` to `0.2.152`.
- update `bindgen` to `0.66.1`.

## [3.10.0] - 2023-09-08
- bump MSRV to v1.65.
  - fixes broken MSRV by downstream `env_logger` dependency.
- remove default features from `bindgen`.
- update dependencies: `libc, `serial_test`.
  - fixes security issue from old downstream `atty` dependency.
- change maintenance status to passively-maintained.
- add `nightly_docs` feature.
- update CI.

## [3.9.1] - 2023-04-13
- fix compilation on MacOs (#27).

## [3.9.0] - 2023-04-04
- `NcPlane.contents` now returns `NcResult`.
- reexport most ffi functions generate by bindgen.
- make `libc` an optional dependency and do not require it for `rstring_free` macro, and several methods. 
- remove macros `sleep`, `nc_render_sleep`, `pile_render_sleep`, `visual_render_sleep`.

## [3.8.0] - 2023-03-06
- The library is now `no_std` by default.

## [3.7.5] - 2023-02-23
## [3.7.4] - 2023-02-23
## [3.7.3] - 2023-02-09
## [3.7.1] - 2023-01-11
## [3.7.0] - 2022-09-27
## [3.6.1] - 2022-06-17
- fix compilation on apple M1.

## [3.6.0] - 2022-06-12
## [3.5.0] - 2022-04-22
## [3.4.0] - 2022-04-08
## [3.3.0] - 2022-03-27
## [3.2.0] - 2022-03-21
## [3.1.2] - 2022-01-10
## [3.0.5] - 2021-12-09
- fix crate compilation in docs.rs

## [3.0.0] - 2021-12-06
## [2.4.8] - 2021-10-23
## [2.4.7] - 2021-10-18
## [2.4.5] - 2021-10-08

[unreleased]: https://github.com/dankamongmen/libnotcurses-sys/compare/v3.10.1...HEAD
[3.10.1]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.10.1
[3.10.0]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.10.0
[3.9.1]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.9.1
[3.9.0]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.9.0
[3.8.0]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.8.0
[3.7.5]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.7.5
[3.7.4]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.7.4
[3.7.3]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.7.3
[3.7.1]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.7.1
[3.7.0]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.7.0
[3.6.1]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.6.1
[3.6.0]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.6.0
[3.5.0]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.5.0
[3.4.0]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.4.0
[3.3.0]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.3.0
[3.2.0]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.2.0
[3.1.2]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.1.2
[3.0.5]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.0.5
[3.0.0]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v3.0.0
[2.4.8]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v2.4.8
[2.4.7]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v2.4.7
[2.4.5]: https://github.com/dankamongmen/libnotcurses-sys/releases/tag/v2.4.5

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
