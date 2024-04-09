// Copyright (c) The datatest-stable Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{path::Path, process::ExitCode};

use crate::{utils, Result};
use camino::{Utf8Path, Utf8PathBuf};
use libtest_mimic::{Arguments, Trial};

#[doc(hidden)]
pub fn runner(requirements: &[Requirements]) -> ExitCode {
    let args = Arguments::from_args();

    let mut tests: Vec<_> = requirements.iter().flat_map(|req| req.expand()).collect();
    tests.sort_unstable_by(|a, b| a.name().cmp(b.name()));

    let conclusion = libtest_mimic::run(&args, tests);
    if conclusion.has_failed() {
        // libtest-mimic uses exit code 101 for test failures -- retain the same behavior.
        101.into()
    } else {
        ExitCode::SUCCESS
    }
}

#[doc(hidden)]
pub struct Requirements {
    test: TestFn,
    test_name: String,
    root: Utf8PathBuf,
    pattern: String,
}

impl Requirements {
    #[doc(hidden)]
    pub fn new<P: TestFnPath + ?Sized>(
        test: fn(&P) -> Result<()>,
        test_name: String,
        root: Utf8PathBuf,
        pattern: String,
    ) -> Self {
        Self {
            test: P::convert(test),
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
                        testfn
                            .call(&path)
                            .map_err(|err| format!("{:?}", err).into())
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

#[derive(Clone, Copy)]
#[doc(hidden)]
pub enum TestFn {
    Path(fn(&Path) -> Result<()>),
    Utf8Path(fn(&Utf8Path) -> Result<()>),
}

mod private {
    pub trait Sealed {}
}

#[doc(hidden)]
pub trait TestFnPath: private::Sealed {
    fn convert(f: fn(&Self) -> Result<()>) -> TestFn;
}

impl private::Sealed for Path {}

impl TestFnPath for Path {
    fn convert(f: fn(&Self) -> Result<()>) -> TestFn {
        TestFn::Path(f)
    }
}

impl private::Sealed for Utf8Path {}

impl TestFnPath for Utf8Path {
    fn convert(f: fn(&Self) -> Result<()>) -> TestFn {
        TestFn::Utf8Path(f)
    }
}

impl TestFn {
    fn call(&self, path: &Utf8Path) -> Result<()> {
        match self {
            TestFn::Path(f) => f(path.as_ref()),
            TestFn::Utf8Path(f) => f(path),
        }
    }
}
