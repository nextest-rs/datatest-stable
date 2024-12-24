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
        // In general we can't verify the contents, but in this case we can do
        // so because the contents are known.
        compare_include_dir_contents(path, contents.as_bytes())
    }

    pub(crate) fn test_artifact_utf8_string(path: &Utf8Path, contents: String) -> Result<()> {
        compare_include_dir_contents(path.as_std_path(), contents.as_bytes())
    }

    pub(crate) fn test_artifact_bytes(path: &Path, contents: Vec<u8>) -> Result<()> {
        compare_include_dir_contents(path, &contents)
    }

    pub(crate) fn test_artifact_utf8_bytes(path: &Utf8Path, contents: Vec<u8>) -> Result<()> {
        compare_include_dir_contents(path.as_std_path(), &contents)
    }

    fn compare_include_dir_contents(path: &Path, expected: &[u8]) -> Result<()> {
        // The path must be relative.
        assert!(path.is_relative(), "path must be relative: {:?}", path);

        // Prepend tests/files to the path to get the expected contents.
        let path = Path::new("tests/files").join(path);
        compare_contents(&path, expected)
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
}

fn compare_contents(path: &Path, expected: &[u8]) -> Result<()> {
    let actual =
        std::fs::read(path).map_err(|error| format!("failed to read file: {:?}: {error}", path))?;

    assert_eq!(expected, &actual, "file contents match for {:?}", path);

    Ok(())
}

datatest_stable::harness!(
    test_artifact,
    "tests/files",
    r"^.*(?<!\.skip)\.txt$", // this regex pattern skips .skip.txt files
    test_artifact_utf8,
    "tests/files",
    r"^.*\.txt$", // this regex pattern matches all files
    with_contents::test_artifact_string,
    maybe_include_dir!(),
    r"^.*\.txt$",
    with_contents::test_artifact_utf8_string,
    &with_contents::MAYBE_INCLUDE_STATIC, // Test out some combinations with &'static include_dir::Dir.
    r"^.*\.txt$",
    with_contents::test_artifact_bytes,
    &with_contents::MAYBE_INCLUDE_STATIC,
    r"^.*\.txt$",
    with_contents::test_artifact_utf8_bytes,
    maybe_include_dir!(),
    r"^.*\.txt$",
);
