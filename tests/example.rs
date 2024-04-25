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

fn test_artifact_string(path: &Path, contents: String) -> Result<()> {
    compare_contents(path, contents.as_bytes())
}

fn test_artifact_utf8_string(path: &Utf8Path, contents: String) -> Result<()> {
    compare_contents(path.as_std_path(), contents.as_bytes())
}

fn test_artifact_bytes(path: &Path, contents: Vec<u8>) -> Result<()> {
    compare_contents(path, &contents)
}

fn test_artifact_utf8_bytes(path: &Utf8Path, contents: Vec<u8>) -> Result<()> {
    compare_contents(path.as_std_path(), &contents)
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
    test_artifact_string,
    "tests/files",
    r"^.*\.txt$",
    test_artifact_utf8_string,
    "tests/files",
    r"^.*\.txt$",
    test_artifact_bytes,
    "tests/files",
    r"^.*\.txt$",
    test_artifact_utf8_bytes,
    "tests/files",
    r"^.*\.txt$",
);
