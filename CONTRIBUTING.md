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

### Bug reports

You can report any bugs [here](https://github.com/LuisMBaezCo/quicktest/issues).