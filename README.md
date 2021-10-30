<p align="center">
  <img width="128" height="128" src="assets/logo/quicktest-512x512.png">
</p>

<h1 align="center">Quick Test CLI</h1>

<p align="center">Command Line Interface (CLI) for Stress Testing for Competitive Programming</p>

[![Current Crates.io Version](https://img.shields.io/crates/v/quicktest.svg)](https://crates.io/crates/quicktest) [![Quick Test](https://circleci.com/gh/LuisMBaezCo/quicktest.svg?style=shield)](https://app.circleci.com/pipelines/github/LuisMBaezCo/quicktest) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-yellow.svg)](https://github.com/LuisMBaezCo/quicktest)

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
        quicktest tle --target-file=main.cpp --gen-file=gen.cpp
        ```

* **Verify the correctness of the code using a verifier script:** Similar to the previous one, this test verifies that the code does not have an incorrect answer for some test cases using a verifier script because there may be many correct answers.
    * **Sample:**
        ```shell
        quicktest check --target-file=main.cpp --checker-file=correct.cpp --gen-file=gen.cpp
        ```

## Commands

**Note:** you can use the long command `quicktest` or the alias `qt`

* `quicktest cmp | qt cmp`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-c=<value> | --correct-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --cases=<value> [default: 1000]`
    * `--timeout=<value>  [default: 2000]` Unit of time: `ms`

---

* `quicktest tle | qt tle`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --cases=<value> [default: 1000]`
    * `--timeout=<value>  [default: 2000]` Unit of time: `ms`
---

* `quicktest check | qt check`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-c=<value> | --checker-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --cases=<value> [default: 1000]`
    * `--timeout=<value>  [default: 2000]` Unit of time: `ms`
---

* **Flags of the `cmp`, `tle` and `check` subcommands**

    * `--break-bad | --break`  Break if WA, TLE or RTE states occurs
    * `--run-ac`     Run test cases Accepted
    * `--run-all`    Run all test cases
    * `--run-rte`    Run test cases Run Time Error
    * `--run-tle`    Run test cases Time Limited Exceeded
    * `--run-wa`     Run test cases Wrong Answer
    * `--save-all`   Save all test cases
    * `--save-bad`   Save only bad cases with WA, TLE or RTE states

---

* `quicktest example | qt example`
    
    **Flags**
    
    * `--check`   Show examples of the check subcommand
    * `--cmp`     Show examples of the cmp subcommand
    * `--tle`     Show examples of the tle subcommand
    
    **Nota:** can only use one flag at a time

## Supported Languages

| Language           |       Version          |
|--------------------|------------------------|
| C++                | -std=c++17             |
| Python             | Version 3              |

## Compilation and Execution Commands

| Language     | Compile / Interpreter                                      | Execution Command          |
|:------------:|------------------------------------------------------------|----------------------------|
| C++17        | `g++ -std=c++17 -Wall -DONLINE_JUDGE=1 -o main main.cpp`   | `./main seed testcase`     |
| Python3      |                                                            | `python3 main.py seed testcase`|

## License
Licensed under either of these:
* MIT license ([LICENSE-MIT](https://github.com/LuisMBaezCo/quicktest/blob/main/LICENSE) or https://opensource.org/licenses/MIT)

<sub>Logo image based on the one made by <a href="https://www.flaticon.com/authors/freepik" id-flaticon="owl_2369306;hexagonal_73861" title="Freepik">Freepik</a> for <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a>