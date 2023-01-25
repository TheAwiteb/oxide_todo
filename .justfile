# List the available commands
_default:
    just --list

# Build the RESTful API
build:
    cargo +stable build --all-features --verbose

# Run register tests
_register_tests:
    dotenv cargo +stable test -j 1 --all-features tests::register

# Run login tests
_login_tests:
    dotenv cargo +stable test -j 1 --all-features tests::login

# Run revoke tests
_revoke_tests:
    dotenv cargo +stable test -j 1 --all-features tests::revoke

# Run the tests
tests:
    just _register_tests
    just _login_tests
    just _revoke_tests

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
