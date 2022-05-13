<p align="center">
  <img width="128" height="128" src="assets/logo/quicktest-512x512.png">
</p>

<h1 align="center">Quick Test CLI</h1>

<p align="center">Command Line Interface (CLI) for Stress Testing for Competitive Programming</p>

[![Current Crates.io Version](https://img.shields.io/crates/v/quicktest.svg)](https://crates.io/crates/quicktest) [![Quick Test](https://circleci.com/gh/LuisMBaezCo/quicktest.svg?style=shield)](https://app.circleci.com/pipelines/github/LuisMBaezCo/quicktest) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue.svg)](https://github.com/LuisMBaezCo/quicktest)

## Documentation

**⚠️ Note:** _Quick Test CLI_ is currently below version `v1.0.0`, so it may contain bugs, you can report any bugs [here](https://github.com/LuisMBaezCo/quicktest/issues).

Read this in other languages: [_Español_](docs/README.es-ES.md), [_Português_](docs/README.pt-BR.md) 

**Table of Contents**

- [Getting Started](#getting-started)
  - [Installation](#installation)
- [Introduction](#introduction)
- [Commands](#commands)
- [Supported Languages](#supported-languages)
- [Compilation and Execution Commands](#compilation-and-execution-commands)
- [License](#license)

## Getting Started

### Installation

If you already have Rust on your system:

```sh
cargo install quicktest
```

If you don't have rust installed on your system, the following command will install Rust and the CLI at once:

Shell (Linux, Mac):
```sh
curl https://sh.rustup.rs -sSf | sh  && cargo install quicktest
```

## Introduction

Quick Test CLI is a project to perform stress testing in competitive programming contests in an easy and fast way, focusing only on the contest.

Currently, Quick Test CLI supports three types of tests which are listed below:

* **Check the correctness of the code compared to a slower version:** Verify that the code does not have incorrect answers for some test cases, using a random generator and a slower version which is sure what is correct with which the answers will be compared.
    * **Sample:**
        ```shell
        quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp
        ```

* **Detect cases with TLE:** Verify that the code execution time does not exceed what is allowed, using a random generator for multiple test cases.
    * **Sample:**
        ```shell
        quicktest stress --target-file=main.cpp --gen-file=gen.cpp
        ```

* **Verify the correctness of the code using a verifier script:** Similar to the previous one, this test verifies that the code does not have an incorrect answer for some test cases using a verifier script because there may be many correct answers.
    * **Sample:**
        ```shell
        quicktest check --target-file=main.cpp --checker-file=correct.cpp --gen-file=gen.cpp
        ```

* **Run a target file with test case files matching a prefix:**
    * **Sample:**
        ```shell
        quicktest output --target-file=main.cpp --prefix=testcase_ac
        ```

## Commands

**Note:** you can use the long command `quicktest` or the alias `qt`

* `quicktest cmp | qt cmp`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-c=<value> | --correct-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --tc=<value> [default: 1000]`
    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--prefix=<value> | -p=<value>` conflict with `--gen-file` (Only one can be used at a time)
    * `--diff`  Show differences between the expected file and the output file

---

* `quicktest stress | qt stress`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --tc=<value> [default: 1000]`
    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--prefix=<value> | -p=<value>` conflict with `--gen-file` (Only one can be used at a time)

---

* `quicktest check | qt check`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-c=<value> | --checker-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --tc=<value> [default: 1000]`
    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--prefix=<value> | -p=<value>` conflict with `--gen-file` (Only one can be used at a time)

---

* **Flags of the `cmp`, `stress` and `check` subcommands**

    * `--break-bad | --break`  Break if WA, TLE or RTE states occurs
    * `--run-ac`     Run test cases Accepted
    * `--run-all`    Run all test cases
    * `--run-rte`    Run test cases Run Time Error
    * `--run-tle`    Run test cases Time Limited Exceeded
    * `--run-wa`     Run test cases Wrong Answer
    * `--save-all`   Save all test cases
    * `--save-bad`   Save only bad cases with WA, TLE or RTE states

---
* `quicktest output | qt output`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-p=<value> | --prefix=<value>`

    **Other Options**

    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--break-bad | --break`  Break if WA, TLE or RTE states occurs
    * `--save-out`   Save the output of the target file for each test case

---
* `quicktest setup | qt setup`

    **Subcommand**

    * `config` Subcommand that allows to change C++ settings

        **Options**

        * `--label=<value>` Label with the path of the configuration that you want to change
        * `--value=<value>` value you want to change a selected label to

---
* `quicktest example | qt example`
    
    **Flags**
    
    * `--check`   Show examples of the check subcommand
    * `--cmp`     Show examples of the cmp subcommand
    * `--stress`  Show examples of the stress subcommand
    * `--output`     Show examples of the output subcommand
    * `--setup`   Show examples of the setup subcommand
    
    **Nota:** can only use one flag at a time

## Supported Languages

| Language           |
|--------------------|
| C++                |
| Python             |
| Rust Lang          |

## Compilation and Execution Commands

| Language | Compile / Interpreter                                                                                                                                                     | Execution Command               |
|----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------|
| C++17    | `g++ -std=c++17 -Wall -DONLINE_JUDGE=1 -o main main.cpp`                                                                                                                  | `./main seed testcase`          |
| Python3  |                                                                                                                                                                           | `python3 main.py seed testcase` |
| Rust     | `cp main.rs ~/.quicktest/rust/src/main.rs && cargo build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust main` | `./main seed testcase`          |

## License
Licensed under either of these:
* MIT license ([LICENSE-MIT](https://github.com/LuisMBaezCo/quicktest/blob/main/LICENSE) or https://opensource.org/licenses/MIT)

---

- <sub>Logo image based on the one made by <a href="https://www.flaticon.com/authors/freepik" id-flaticon="owl_2369306;hexagonal_73861" title="Freepik">Freepik</a> for <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a>

- <sub>Documentation was based on <a href="https://searleser97.github.io/cpbooster/" title="Flaticon">cpbooster</a>