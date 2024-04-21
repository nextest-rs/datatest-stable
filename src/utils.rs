// Copyright (c) The datatest-stable Contributors
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

pub fn derive_test_path(root: &Utf8Path, filter: &str, test_name: &str) -> Option<Utf8PathBuf> {
    let relative = filter.strip_prefix(test_name)?.strip_prefix("::")?;
    Some(root.join(relative))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod derive_path {
        use super::*;

        #[test]
        fn missing_test_name() {
            assert_eq!(derive_test_path("root".into(), "file", "test_name"), None);
        }

        #[test]
        fn missing_colons() {
            assert_eq!(
                derive_test_path("root".into(), "test_name", "test_name"),
                None
            );
        }

        #[test]
        fn is_relative_to_root() {
            assert_eq!(
                derive_test_path("root".into(), "test_name::file", "test_name"),
                Some("root/file".into())
            );
            assert_eq!(
                derive_test_path("root2".into(), "test_name::file", "test_name"),
                Some("root2/file".into())
            );
        }

        #[test]
        fn nested_dirs() {
            assert_eq!(
                derive_test_path("root".into(), "test_name::dir/dir2/file", "test_name"),
                Some("root/dir/dir2/file".into())
            );
        }

        #[test]
        fn subsequent_module_separators_remain() {
            assert_eq!(
                derive_test_path("root".into(), "test_name::mod::file", "test_name"),
                Some("root/mod::file".into())
            );
        }

        #[test]
        fn inverse_of_derive_test_name() {
            let root: Utf8PathBuf = "root".into();
            for (path, test_name) in [
                (root.join("foo/bar.txt"), "test_name"),
                (root.join("foo::bar.txt"), "test_name"),
                (root.join("foo/bar/baz"), "test_name"),
                (root.join("foo"), "test_name::mod"),
                (root.join("🦀"), "🚀::🚀"),
            ] {
                let derived_test_name = derive_test_name(&root, &path, test_name);
                assert_eq!(
                    derive_test_path(&root, &derived_test_name, test_name),
                    Some(path)
                );
            }
        }
    }
}
