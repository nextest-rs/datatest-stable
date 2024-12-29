// Copyright (c) The datatest-stable Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use camino::Utf8Path;
use datatest_stable::Result;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::Path;

static SETUP_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TEARDOWN_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TEST_ARTIFACT_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TEST_ARTIFACT_UTF8_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn test_setup() -> Result<()> {
    SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("Setup called");
    Ok(())
}

fn test_teardown(exit_code: std::process::ExitCode) -> Result<()> {
    TEARDOWN_COUNTER.fetch_add(1, Ordering::SeqCst);
    if exit_code == std::process::ExitCode::SUCCESS {
        println!("Teardown called after success");
    } else {
        println!("Teardown called after failure");
    }
    Ok(())
}

fn test_artifact(_path: &Path) -> Result<()> {
    TEST_ARTIFACT_COUNTER.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

fn test_artifact_utf8(_path: &Utf8Path) -> Result<()> {
    TEST_ARTIFACT_UTF8_COUNTER.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

datatest_stable::harness! {
    setup = test_setup,
    teardown = test_teardown,
    {
        test = test_artifact,
        root = "tests/files",
    },
    {
        test = test_artifact_utf8,
        root = "tests/files",
    }
}

#[test]
fn verify_counters() {
    // Setup should be called exactly once.
    assert_eq!(SETUP_COUNTER.load(Ordering::SeqCst), 1);

    // Teardown should be called exactly once.
    assert_eq!(TEARDOWN_COUNTER.load(Ordering::SeqCst), 1);

    // Each test function should be called once per file in tests/files.
    let file_count = fs::read_dir("tests/files").unwrap().count();
    assert_eq!(TEST_ARTIFACT_COUNTER.load(Ordering::SeqCst), file_count);
    assert_eq!(TEST_ARTIFACT_UTF8_COUNTER.load(Ordering::SeqCst), file_count);
}
