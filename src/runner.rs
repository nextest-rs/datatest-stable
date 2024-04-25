// Copyright (c) The datatest-stable Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{utils, Result};
use camino::{Utf8Path, Utf8PathBuf};
use libtest_mimic::{Arguments, Trial};
use std::{path::Path, process::ExitCode};

#[doc(hidden)]
pub fn runner(requirements: &[Requirements]) -> ExitCode {
    if let Some(cwd) = custom_cwd() {
        std::env::set_current_dir(&cwd).expect("set custom working directory");
    }

    let args = Arguments::from_args();

    let tests = find_tests(&args, requirements);

    let conclusion = libtest_mimic::run(&args, tests);

    // This used to use `Conclusion::exit`, but that exits the process via `std::process::exit` as
    // of libtest-mimic 0.7.0. This breaks some things, e.g. llvm-cov on Windows.
    // https://github.com/nextest-rs/datatest-stable/issues/20
    //
    // Use `std::process::ExitCode` instead, and return it in main.

    conclusion.exit_code()
}

/// One of our tests requires that a custom working directory be set. This function is used to do
/// that.
fn custom_cwd() -> Option<Utf8PathBuf> {
    std::env::var("__DATATEST_CWD").ok().map(Utf8PathBuf::from)
}

fn find_tests(args: &Arguments, requirements: &[Requirements]) -> Vec<Trial> {
    let tests: Vec<_> = if let Some(exact_filter) = exact_filter(args) {
        let exact_tests: Vec<_> = requirements
            .iter()
            .flat_map(|req| req.exact(exact_filter))
            .collect();

        match NextestKind::determine() {
            NextestKind::InUse { process_per_test } => {
                if exact_tests.is_empty() {
                    panic!("Failed to find exact match for filter {exact_filter}");
                }
                if process_per_test && exact_tests.len() > 1 {
                    panic!(
                        "Only expected one but found {} exact matches for filter {exact_filter}",
                        exact_tests.len()
                    );
                }
            }
            NextestKind::NotInUse => {}
        }

        exact_tests
    } else if is_full_scan_forbidden(args) {
        panic!("Exact filter was expected to be used");
    } else {
        let mut tests: Vec<_> = requirements.iter().flat_map(|req| req.expand()).collect();
        tests.sort_unstable_by(|a, b| a.name().cmp(b.name()));
        tests
    };
    tests
}

#[derive(Clone, Copy, Debug)]
enum NextestKind {
    NotInUse,
    InUse { process_per_test: bool },
}

impl NextestKind {
    fn determine() -> Self {
        if std::env::var("NEXTEST").as_deref() == Ok("1") {
            // Process-per-test means that exactly one test should be run.
            let process_per_test =
                std::env::var("NEXTEST_EXECUTION_MODE").as_deref() == Ok("process-per-test");
            Self::InUse { process_per_test }
        } else {
            Self::NotInUse
        }
    }
}

fn is_full_scan_forbidden(args: &Arguments) -> bool {
    !args.list && std::env::var("__DATATEST_FULL_SCAN_FORBIDDEN").as_deref() == Ok("1")
}

fn exact_filter(args: &Arguments) -> Option<&str> {
    if args.exact && args.skip.is_empty() {
        args.filter.as_deref()
    } else {
        None
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

    fn trial(&self, path: Utf8PathBuf) -> Trial {
        let testfn = self.test;
        let name = utils::derive_test_name(&self.root, &path, &self.test_name);
        Trial::test(name, move || {
            testfn
                .call(&path)
                .map_err(|err| format!("{:?}", err).into())
        })
    }

    fn exact(&self, filter: &str) -> Option<Trial> {
        let path = utils::derive_test_path(&self.root, filter, &self.test_name)?;
        path.exists().then(|| self.trial(path))
    }

    /// Scans all files in a given directory, finds matching ones and generates a test descriptor
    /// for each of them.
    fn expand(&self) -> Vec<Trial> {
        let re = fancy_regex::Regex::new(&self.pattern)
            .unwrap_or_else(|_| panic!("invalid regular expression: '{}'", self.pattern));

        let tests: Vec<_> = utils::iterate_directory(&self.root)
            .filter_map(|path_res| {
                let path = path_res.expect("error while iterating directory");
                if re.is_match(path.as_str()).unwrap_or_else(|error| {
                    panic!(
                        "error matching pattern '{}' against path '{}' : {}",
                        self.pattern,
                        path.as_str(),
                        error
                    )
                }) {
                    Some(self.trial(path))
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
