---
sidebar_position: 1
title: Supported Languages
sidebar_label: Supported Languages
---

| Language           |       Version          |
|--------------------|------------------------|
| C++                | -std=c++17             |
| Python             | Version 3              |
| Rust               | edition=2021           |

## Compilation and Execution Commands

| Language | Compile / Interpreter                                                                                                                                                     | Execution Command               |
|----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------|
| C++17    | `g++ -std=c++17 -Wall -DONLINE_JUDGE=1 -o main main.cpp`                                                                                                                  | `./main seed testcase`          |
| Python3  |                                                                                                                                                                           | `python3 main.py seed testcase` |
| Rust     | `cp main.rs ~/.quicktest/rust/src/main.rs && cargo build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust main` | `./main seed testcase`          |

