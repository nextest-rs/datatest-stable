// Copyright (c) The datatest-stable Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

/// `datatest-stable` test harness entry point. Should be declared in the test module.
///
/// Also, `harness` should be set to `false` for that test module in `Cargo.toml` (see [Configuring
/// a target](https://doc.rust-lang.org/cargo/reference/manifest.html#configuring-a-target)).
#[macro_export]
macro_rules! harness {
    ( $( { test = $name:path, root = $root:expr, pattern = $pattern:expr $(,)* } ),+ $(,)* ) => {
        fn main() -> ::std::process::ExitCode {
            let mut requirements = Vec::new();
            use $crate::data_source_kinds::*;
            use $crate::test_kinds::*;

            $(
                requirements.push(
                    $crate::Requirements::new(
                        $name.kind().resolve($name),
                        stringify!($name).to_string(),
                        $root.resolve_data_source(),
                        $pattern.to_string()
                    )
                );
            )+

            $crate::runner(&requirements)
        }
    };
    ( $( $name:path, $root:expr, $pattern:expr ),+ $(,)* ) => {
        // This is the old format with datatest-stable 0.2. Print a nice message
        // in this case.
        const _: () = {
            compile_error!(
concat!(r"this format is no longer supported -- please switch to specifying as:

datatest_stable::harness! {
",
    $(concat!("    { test = ", stringify!($name), ", root = ", stringify!($root), ", pattern = ", stringify!($pattern), " },\n"),)+
r"}

note: patterns are now evaluated relative to the provided root, not to the crate root
"));
        };
    }
}
