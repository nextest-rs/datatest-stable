// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::integer_arithmetic)]

use crate::{utils, Result};
use camino::{Utf8Path, Utf8PathBuf};
use libtest_mimic::{Arguments, Trial};

#[doc(hidden)]
pub fn runner(requirements: &[Requirements]) {
    let args = Arguments::from_args();

    let mut tests: Vec<_> = requirements.iter().flat_map(|req| req.expand()).collect();
    tests.sort_unstable_by(|a, b| a.name().cmp(b.name()));

    libtest_mimic::run(&args, tests).exit()
}

#[doc(hidden)]
pub struct Requirements {
    test: fn(&Utf8Path) -> Result<()>,
    test_name: String,
    root: Utf8PathBuf,
    pattern: String,
}

impl Requirements {
    #[doc(hidden)]
    pub fn new(
        test: fn(&Utf8Path) -> Result<()>,
        test_name: String,
        root: Utf8PathBuf,
        pattern: String,
    ) -> Self {
        Self {
            test,
            test_name,
            root,
            pattern,
        }
    }

    /// Scans all files in a given directory, finds matching ones and generates a test descriptor
    /// for each of them.
    fn expand(&self) -> Vec<Trial> {
        let re = regex::Regex::new(&self.pattern)
            .unwrap_or_else(|_| panic!("invalid regular expression: '{}'", self.pattern));

        let tests: Vec<_> = utils::iterate_directory(&self.root)
            .filter_map(|path_res| {
                let path = path_res.expect("error while iterating directory");
                if re.is_match(path.as_str()) {
                    let testfn = self.test;
                    let name = utils::derive_test_name(&self.root, &path, &self.test_name);
                    Some(Trial::test(name, move || {
                        (testfn)(&path).map_err(|err| format!("{:?}", err).into())
                    }))
                } else {
                    None
                }
            })
            .collect();

        // We want to avoid silent fails due to typos in regexp!
        if tests.is_empty() {
            panic!(
                "no test cases found for test '{}'. Scanned directory: '{}' with pattern '{}'",
                self.test_name, self.root, self.pattern,
            );
        }

        tests
    }
}
