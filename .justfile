# List the available commands
_default:
    just --list

# Build the RESTful API
build:
    cargo +1.65.0 build --all-features --verbose

# Run register tests
_register_tests:
    dotenv cargo +1.65.0 test -j 1 --all-features tests::register:: -- --test-threads 1

# Run login tests
_login_tests:
    dotenv cargo +1.65.0 test -j 1 --all-features tests::login:: -- --test-threads 1

# Run revoke tests
_revoke_tests:
    dotenv cargo +1.65.0 test -j 1 --all-features tests::revoke:: -- --test-threads 1

# Run create todo tests
_create_todo_tests:
    dotenv cargo +1.65.0 test -j 1 --all-features tests::todo::create_todo:: -- --test-threads 1

# Run list todo tests
_list_todo_tests:
    dotenv cargo +1.65.0 test -j 1 --all-features tests::todo::list_todo:: -- --test-threads 1

# Run get todo tests
_get_todo_tests:
    dotenv cargo +1.65.0 test -j 1 --all-features tests::todo::get_todo:: -- --test-threads 1

# Run delete todo tests
_delete_todo_tests:
    dotenv cargo +1.65.0 test -j 1 --all-features tests::todo::delete_todo:: -- --test-threads 1

# Run delete todos tests
_delete_todos_tests:
    dotenv cargo +1.65.0 test -j 1 --all-features tests::todo::delete_todos:: -- --test-threads 1

# Run update todo tests
_update_todo_tests:
    dotenv cargo +1.65.0 test -j 1 --all-features tests::todo::update_todo:: -- --test-threads 1

# Run the tests
tests:
    # Clean the database
    echo > db.sqlite3
    # Run the tests sequentially, because they are not independent
    just _register_tests
    just _login_tests
    just _revoke_tests
    just _create_todo_tests
    just _list_todo_tests
    just _get_todo_tests
    just _delete_todo_tests
    just _delete_todos_tests
    just _update_todo_tests

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
