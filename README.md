<h1 align="center">🧰 Quick Test CLI</h1>

<p align="center">Command Line Interface (CLI) for Stress Testing for Competitive Programming</p>

[![Current Crates.io Version](https://img.shields.io/crates/v/quicktest.svg)](https://crates.io/crates/quicktest) [![Quick Test](https://circleci.com/gh/LuisMBaezCo/quicktest.svg?style=shield)](https://app.circleci.com/pipelines/github/LuisMBaezCo/quicktest) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![](https://img.shields.io/crates/d/quicktest)](https://crates.io/crates/quicktest) [![](https://img.shields.io/badge/platform-Linux%20%7C%20macOS-blue.svg)](https://github.com/LuisMBaezCo/quicktest) [![](https://img.shields.io/badge/rustc-1.51+-yellow.svg)](https://www.rust-lang.org/tools/install) 

## Documentation

**⚠️ Note:** _Quick Test CLI_ is currently below version `v1.0.0`, so it may contain bugs, you can report any bugs [here](https://github.com/LuisMBaezCo/quicktest/issues).

Read this in other languages: [_Español_](docs/README.es-ES.md), [_Português_](docs/README.pt-BR.md) 

**Table of Contents**

- [Introduction](#introduction)
- [Getting Started](#getting-started)
  - [Installation](#installation)
- [Supported Languages](#supported-languages)
- [License](#license)

## Introduction

Quick Test CLI is a project to perform stress testing in competitive programming contests in an easy and fast way, focusing only on the contest.

Currently, Quick Test CLI supports three types of tests which are listed below:

* **Detect cases with TLE:** Verify that the code execution time does not exceed what is allowed, using a random generator for multiple test cases.
    * **Sample:**
        ```shell
        quicktest tle --target-file=”main.cpp” --gen-file=”gen.cpp” --timeout=2000 --test-cases=1000
        ```

* **Check the correctness of the code compared to a slower version:** Verify that the code does not have incorrect answers for some test cases, using a random generator and a slower version which is sure what is correct with which the answers will be compared.
    * **Sample:**
        ```shell
        quicktest cmp --target-file=”main.cpp” --correct-file=”correct.cpp” --gen-file=”gen.cpp” --timeout=2000 --test-cases=1000
        ```

* **Verify the correctness of the code using a verifier script:** Similar to the previous one, this test verifies that the code does not have an incorrect answer for some test cases using a verifier script because there may be many correct answers.
    * **Sample:**
        ```shell
        quicktest check --target-file=”main.cpp” --checker-file=”correct.cpp” --gen-file=”gen.cpp” --timeout=2000 --test-cases=1000
        ```

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

## Supported Languages

| Language           |       Version          |
|--------------------|------------------------|
| C++                | -std=c++17             |
| Python             | Version 3              |


## License
Licensed under either of these:
* MIT license ([LICENSE-MIT](https://github.com/LuisMBaezCo/quicktest/blob/main/LICENSE) or https://opensource.org/licenses/MIT)