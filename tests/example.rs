// Copyright (c) The datatest-stable Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use camino::Utf8Path;
use datatest_stable::Result;
use std::{fs::File, io::Read, path::Path};

fn test_artifact(path: &Path) -> Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(())
}

fn test_artifact_utf8(path: &Utf8Path) -> Result<()> {
    test_artifact(path.as_ref())
}

#[cfg(feature = "include-dir")]
#[macro_use]
mod with_contents {
    use super::*;

    /// Returns an `include_dir::Dir` instance.
    macro_rules! maybe_include_dir {
        () => {
            include_dir::include_dir!("tests/files")
        };
    }

    /// A `&'static include_dir::Dir` instance.
    pub(crate) static MAYBE_INCLUDE_STATIC: include_dir::Dir =
        include_dir::include_dir!("tests/files");

    pub(crate) fn test_artifact_string(path: &Path, contents: String) -> Result<()> {
        compare_contents(path, contents.as_bytes())
    }

    pub(crate) fn test_artifact_utf8_string(path: &Utf8Path, contents: String) -> Result<()> {
        compare_contents(path.as_std_path(), contents.as_bytes())
    }

    pub(crate) fn test_artifact_bytes(path: &Path, contents: Vec<u8>) -> Result<()> {
        compare_contents(path, &contents)
    }

    pub(crate) fn test_artifact_utf8_bytes(path: &Utf8Path, contents: Vec<u8>) -> Result<()> {
        compare_contents(path.as_std_path(), &contents)
    }

    fn compare_contents(path: &Path, expected: &[u8]) -> Result<()> {
        // The path must not begin with "tests/files".
        assert!(
            !path.to_string_lossy().starts_with("tests/files"),
            "path must not start with 'tests/files': {:?}",
            path
        );

        // Prepend tests/files to the path to get the expected contents. In
        // general we can't verify the contents, but in this case we can do so
        // because the paths are also available on disk.
        let path = format!("tests/files/{}", path.to_str().unwrap());
        compare(path.as_ref(), expected)
    }
}

#[cfg(not(feature = "include-dir"))]
#[macro_use]
mod with_contents {
    use super::*;

    /// Returns an `include_dir::Dir` instance.
    macro_rules! maybe_include_dir {
        () => {
            "tests/files"
        };
    }

    /// A `&'static include_dir::Dir` instance.
    pub(crate) static MAYBE_INCLUDE_STATIC: &str = "tests/files";

    pub(crate) fn test_artifact_string(path: &Path, contents: String) -> Result<()> {
        compare_contents(path, contents.as_bytes())
    }

    pub(crate) fn test_artifact_utf8_string(path: &Utf8Path, contents: String) -> Result<()> {
        compare_contents(path.as_std_path(), contents.as_bytes())
    }

    pub(crate) fn test_artifact_bytes(path: &Path, contents: Vec<u8>) -> Result<()> {
        compare_contents(path, &contents)
    }

    pub(crate) fn test_artifact_utf8_bytes(path: &Utf8Path, contents: Vec<u8>) -> Result<()> {
        compare_contents(path.as_std_path(), &contents)
    }

    fn compare_contents(path: &Path, expected: &[u8]) -> Result<()> {
        // The path must begin with "tests/files".
        assert!(
            path.to_string_lossy().starts_with("tests/files"),
            "path must start with 'tests/files': {:?}",
            path
        );
        compare(&path, expected)
    }
}

fn compare(path: &Path, expected: &[u8]) -> Result<()> {
    // The path must be relative.
    assert!(path.is_relative(), "path must be relative: {:?}", path);

    // The path must not have any backslashes on Windows.
    assert!(
        !path.to_string_lossy().contains('\\'),
        "path must not contain backslashes: {:?}",
        path
    );

    let actual =
        std::fs::read(path).map_err(|error| format!("failed to read file: {:?}: {error}", path))?;

    assert_eq!(expected, &actual, "file contents match for {:?}", path);

    Ok(())
}

#[cfg(windows)]
static TESTS_FILES_MAIN_SEP: &str = "tests\\files";

#[cfg(not(windows))]
static TESTS_FILES_MAIN_SEP: &str = "tests/files";

datatest_stable::harness!(
    test_artifact,
    "tests/files",
    r"^.*(?<!\.skip)\.txt$", // this regex pattern skips .skip.txt files
    // ---
    test_artifact_utf8,
    TESTS_FILES_MAIN_SEP, // ensure that tests\files is normalized to tests/files on Windows
    r"^.*\.txt$",         // this regex pattern matches all files
    // ---
    with_contents::test_artifact_string,
    maybe_include_dir!(),
    // This regex matches exactly a.txt, b.txt, and c.skip.txt -- this ensures
    // that patterns are relative to the include dir and not the crate root
    r"^(a|b|c\.skip)\.txt$",
    // ---
    with_contents::test_artifact_utf8_string,
    &with_contents::MAYBE_INCLUDE_STATIC, // Test out some combinations with &'static include_dir::Dir.
    r"^.*\.txt$",
    // ---
    with_contents::test_artifact_bytes,
    &with_contents::MAYBE_INCLUDE_STATIC,
    r"^.*\.txt$",
    // ---
    with_contents::test_artifact_utf8_bytes,
    maybe_include_dir!(),
    r"^.*\.txt$",
);
