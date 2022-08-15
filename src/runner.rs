// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::integer_arithmetic)]

use crate::{utils, Result};
use libtest_mimic::{run_tests, Arguments, Outcome, Test};
use std::path::Path;

#[doc(hidden)]
pub fn runner(requirements: &[Requirements]) {
    let args = Arguments::from_args();

    let mut tests: Vec<_> = requirements.iter().flat_map(|req| req.expand()).collect();
    tests.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    run_tests(&args, tests, |test| (test.data)()).exit()
}

#[doc(hidden)]
pub struct Requirements {
    test: fn(&Path) -> Result<()>,
    test_name: String,
    root: String,
    pattern: String,
}

impl Requirements {
    #[doc(hidden)]
    pub fn new(
        test: fn(&Path) -> Result<()>,
        test_name: String,
        root: String,
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
    fn expand(&self) -> Vec<Test<Box<dyn Fn() -> Outcome + Send + Sync>>> {
        let root = Path::new(&self.root).to_path_buf();

        let re = regex::Regex::new(&self.pattern)
            .unwrap_or_else(|_| panic!("invalid regular expression: '{}'", self.pattern));

        let tests: Vec<_> = utils::iterate_directory(&root)
            .filter_map(|path| {
                let input_path = path.to_string_lossy();
                if re.is_match(&input_path) {
                    let testfn = self.test;
                    let name = utils::derive_test_name(&root, &path, &self.test_name);
                    let testfn: Box<dyn Fn() -> Outcome + Send + Sync> =
                        Box::new(move || match (testfn)(&path) {
                            Ok(()) => Outcome::Passed,
                            Err(err) => Outcome::Failed {
                                msg: Some(format!("Error: {:?}", err)),
                            },
                        });

                    Some(Test {
                        name,
                        kind: String::new(),
                        is_ignored: false,
                        is_bench: false,
                        data: testfn,
                    })
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
