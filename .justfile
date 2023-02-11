# List the available commands
_default:
    just --list

# Build the RESTful API
build:
    cargo +1.65.0 build --all-features --verbose

# Run register tests
_register_tests:
    dotenv run cargo +1.65.0 test -j 1 --all-features tests::register

# Run login tests
_login_tests:
    dotenv run cargo +1.65.0 test -j 1 --all-features tests::login

# Run revoke tests
_revoke_tests:
    dotenv run cargo +1.65.0 test -j 1 --all-features tests::revoke

# Run the tests
tests:
    # Clean the database
    echo > db.sqlite3
    just _register_tests
    just _login_tests
    just _revoke_tests

# Format everything
fmt:
    cargo +1.65.0 fmt --all --verbose

# Check the format of everything
fmt-check:
    cargo +1.65.0 fmt --all --check --verbose

# Run Rust linter (clippy)
linter:
    cargo +1.65.0 clippy --workspace --examples --all-features --verbose

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
