// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use camino::{Utf8Path, Utf8PathBuf};

/// Helper function to iterate through all the files in the given directory, skipping hidden files,
/// and return an iterator of their paths.
pub fn iterate_directory(path: &Utf8Path) -> impl Iterator<Item = std::io::Result<Utf8PathBuf>> {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter(|res| {
            // Continue to bubble up all errors to the parent.
            res.as_ref().map_or(true, |entry| {
                entry.file_type().is_file()
                    && entry
                        .file_name()
                        .to_str()
                        .map_or(false, |s| !s.starts_with('.')) // Skip hidden files
            })
        })
        .map(|res| match res {
            Ok(entry) => {
                Utf8PathBuf::try_from(entry.into_path()).map_err(|error| error.into_io_error())
            }
            Err(error) => Err(error.into()),
        })
}

pub fn derive_test_name(root: &Utf8Path, path: &Utf8Path, test_name: &str) -> String {
    let relative = path
        .strip_prefix(root)
        .unwrap_or_else(|_| panic!("failed to strip prefix '{}' from path '{}'", root, path));

    format!("{}::{}", test_name, relative)
}
