// Copyright (c) The datatest-stable Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#[test]
fn run_example() {
    let output = std::process::Command::new("cargo")
        .args(["nextest", "run", "--test=example", "--color=never"])
        .env("__DATATEST_FULL_SCAN_FORBIDDEN", "1")
        .output()
        .expect("Failed to run `cargo nextest`");

    // It's a pain to make assertions on byte slices (even a subslice check isn't easy)
    // and it's also difficult to print nice error messages. So we just assume all
    // nextest output will be utf8 and convert it.
    let stderr = std::str::from_utf8(&output.stderr).expect("cargo nextest stderr should be utf-8");

    assert!(
        output.status.success(),
        "Command failed (exit status: {}, stderr: {stderr})",
        output.status
    );

    let lines: &[&str] = &[
        "datatest-stable::example test_artifact::::colon::dir/::.txt",
        "datatest-stable::example test_artifact::::colon::dir/a.txt",
        "datatest-stable::example test_artifact::a.txt",
        "datatest-stable::example test_artifact_utf8::::colon::dir/::.txt",
        "datatest-stable::example test_artifact::b.txt",
        "datatest-stable::example test_artifact_utf8::::colon::dir/a.txt",
        "datatest-stable::example test_artifact_utf8::a.txt",
        "datatest-stable::example test_artifact_utf8::c.skip.txt",
        "datatest-stable::example test_artifact_utf8::b.txt",
        "9 tests run: 9 passed, 0 skipped",
    ];

    for line in lines {
        assert!(
            stderr.contains(line),
            "Expected to find substring\n  {line}\nin stderr\n  {stderr}",
        );
    }
}
