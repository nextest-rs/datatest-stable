// Copyright (c) The datatest-stable Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

/// `datatest-stable` test harness entry point. Should be declared in the test module.
///
/// Also, `harness` should be set to `false` for that test module in `Cargo.toml` (see [Configuring
/// a target](https://doc.rust-lang.org/cargo/reference/manifest.html#configuring-a-target)).
#[macro_export]
macro_rules! harness {
    ( $( $name:path, $root:expr, $pattern:expr ),+ $(,)* ) => {
        fn main() -> ::std::process::ExitCode {
            let mut requirements = Vec::new();

            $(
                requirements.push(
                    $crate::Requirements::new(
                        $name,
                        stringify!($name).to_string(),
                        $root.to_string().into(),
                        $pattern.to_string()
                    )
                );
            )+

            $crate::runner(&requirements)
        }
    };
}
