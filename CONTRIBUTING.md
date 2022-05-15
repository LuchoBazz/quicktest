# Contributing to Quick Test CLI

1. Fork this repository and clone it locally: `git clone https://github.com/{yourUsername}/quicktest`
2. `cd` (change directory) to `quicktest` repo directory
3. Install dependencies: run npm install
4. Install `quicktest` from source: `cargo build`
5. Make code changes

### Before making a Pull Request

you must execute the following commands

```shell
cargo fmt
cargo build
cargo clippy -- -D warnings
./target/debug/quicktest setup config --label="Language::Python.PROGRAM" --value="python3"
cargo test stress_subcommand -- --test-threads 1
cargo test cmp_subcommand -- --test-threads 1
cargo test check_subcommand -- --test-threads 1
cargo test output_subcommand -- --test-threads 1
cargo test setup_subcommand -- --test-threads 1
```

### How can you contribute?

- Improve documentation, since it was written by a non-native English speaker and may contain grammatical errors

- Add new documentation

- add unit tests

- add support for new languages

- add new examples in the [`/examples`](https://github.com/LuisMBaezCo/quicktest/tree/main/examples) folder in the different supported languages

- reporting undetected errors

- suggest new features to implement

### Future Updates (work not started yet, you can contribute by coding any of these features)

- add `inter` subcommand to support testing for interactive problems
- use [tokyo](https://crates.io/crates/tokio) y [futures](https://crates.io/crates/futures) crates to use async functions
- Solve TODOs in code.