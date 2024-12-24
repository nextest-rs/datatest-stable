# Print a help message.
help:
    just --list


# Run `cargo hack --feature-powerset` with the given arguments.
powerset *args:
    cargo hack --feature-powerset {{args}}

# Build docs for crates and direct dependencies
rustdoc:
    @cargo tree --depth 1 -e normal --prefix none --workspace \
        | gawk '{ gsub(" v", "@", $0); printf("%s\n", $1); }' \
        | xargs printf -- '-p %s\n' \
        | RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS='--cfg=doc_cfg' xargs cargo doc --no-deps --lib --all-features

# Generate README.md files using `cargo-sync-rdme`.
generate-readmes:
    cargo sync-rdme --toolchain nightly --all-features
