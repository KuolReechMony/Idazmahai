# Guidelines

## Commits

- Project must meet following conditions before each commit:
1. Formatted: `cargo fmt --check`
2. Linted: `cargo clippy --all-targets --all-features -- -D warnings`
3. Tested with all tests passing: `cargo test`
4. Compiles: `cargo check --all-targets --all-features`

