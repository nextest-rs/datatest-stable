# Changelog

## [0.2.0] - 2023-08-29

### Changed

- The test signature is now `fn(&`[`Utf8Path`]`)` rather than `fn(&Path)`. If necessary, a `Utf8Path` can be converted to a `&Path` with [`.as_ref()`] or [`.as_std_path()`].
- Non-Unicode paths now consistently produce errors. Previously, the treatment of such paths was inconsistent -- they would either be skipped or produce errors.
- Internal dependency update: libtest-mimic updated to version 0.6.1.
- MSRV updated to Rust 1.60.

[`Utf8Path`]: https://docs.rs/camino/latest/camino/struct.Utf8Path.html
[`.as_ref()`]: https://docs.rs/camino/latest/camino/struct.Utf8Path.html#impl-AsRef%3COsStr%3E-for-Utf8Path
[`.as_std_path()`]: https://docs.rs/camino/latest/camino/struct.Utf8Path.html#method.as_std_path

## [0.1.3] - 2022-08-15

### Changed

- Errors are now displayed with the `Debug` implementation, which prints out the full error chain
  with libraries like `anyhow` or `eyre`, rather than the `Display` implementation. Thanks
  [Alex Badics] for your first contribution!
- MSRV updated to Rust 1.58.

### Internal improvements

- datatest-stable now uses libtest-mimic 0.5.2. Thanks [Lukas Kalbertodt] (maintainer of
  libtest-mimic) for your first contribution!

[Alex Badics]: https://github.com/badicsalex
[Lukas]: https://github.com/LukasKalbertodt

## [0.1.2] - 2022-05-22

### Changed

- New internal implementation, based on top of [libtest-mimic](https://github.com/LukasKalbertodt/libtest-mimic).
- Repository updated to [nextest-rs/datatest-stable](https://github.com/nextest-rs/datatest-stable).
- MSRV updated to Rust 1.56.

There are no functional changes in this release.

## [0.1.1] - 2021-04-16

### Added

- Initial release with basic support for data-driven tests.

(Version 0.1.0 was yanked because of a metadata issue.)

[0.2.0]: https://github.com/nextest-rs/datatest-stable/releases/tag/datatest-stable-0.2.0
[0.1.3]: https://github.com/nextest-rs/datatest-stable/releases/tag/datatest-stable-0.1.3
[0.1.2]: https://github.com/nextest-rs/datatest-stable/releases/tag/datatest-stable-0.1.2
[0.1.1]: https://github.com/nextest-rs/datatest-stable/releases/tag/datatest-stable-0.1.1
