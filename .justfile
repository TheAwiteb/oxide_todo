# List the available commands
_default:
    just --list

# Build the RESTful API
build:
    cargo +stable build --all-features --verbose

# Run the tests
tests:
    cargo +stable test --all-features --verbose

# Format everything
fmt:
    cargo +stable fmt --all --verbose

# Check the format of everything
fmt-check:
    cargo +stable fmt --all --check --verbose

# Run Rust linter (clippy)
linter:
    cargo +stable clippy --workspace --examples --all-features --verbose

# Run the CI
ci: build && fmt-check linter tests


[private]
alias b := build
[private]
alias t := tests
[private]
alias f := fmt
[private]
alias format := fmt
[private]
alias fc := fmt-check
[private]
alias check-fmt := fmt-check
[private]
alias l := linter
