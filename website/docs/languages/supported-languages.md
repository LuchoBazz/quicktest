---
sidebar_position: 1
title: Supported Languages
sidebar_label: Supported Languages
---

| Language           |
|--------------------|
| C++                |
| Python             |
| Rust Lang          |
| Go Lang            |

## Compilation and Execution Commands

| Language  | Compile / Interpreter                                                                                                                                                     | Execution Command               |
|-----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------|
| C++17     | `g++ -std=c++17 -Wall -DONLINE_JUDGE=1 -o main main.cpp`                                                                                                                  | `./main seed testcase`          |
| Python3   |                                                                                                                                                                           | `python3 main.py seed testcase` |
| Rust Lang | `cp main.rs ~/.quicktest/rust/src/main.rs && cargo build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust main` | `./main seed testcase`          |
| Go Lang   | `cp main.go ~/.quicktest/go_mod/main.go && go build -buildmode=exe -o ./.qtest/main ~/.quicktest/go_mod/main.go`                                                          | `./main seed testcase`          |


