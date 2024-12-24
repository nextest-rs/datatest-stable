# Print a help message.
help:
    just --list

# Build docs for crates and direct dependencies
rustdoc:
    @cargo tree --depth 1 -e normal --prefix none --workspace \
        | gawk '{ gsub(" v", "@", $0); printf("%s\n", $1); }' \
        | xargs printf -- '-p %s\n' \
        | RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS='--cfg=doc_cfg' xargs cargo doc --no-deps --lib --all-features

# Generate README.md files from README.tpl and lib.rs files
generate-readmes:
    #!/usr/bin/env bash
    set -eo pipefail

    git ls-files | grep README.tpl$ | while read -r readme; do
        echo "Generating README for $readme"
        dir=$(dirname "$readme")
        cargo readme --project-root "$dir" > "$dir/README.md.tmp"
        gawk -f "scripts/fix-readmes.awk" "$dir/README.md.tmp" > "$dir/README.md"
        rm "$dir/README.md.tmp"
    done
