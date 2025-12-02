---
sidebar_position: 1
title: Supported Languages
sidebar_label: Supported Languages
---

| Language           |
|--------------------|
| C++                |
| Java               |
| Python             |
| Rust Lang          |
| Go Lang            |
| GNU C              |
| Kotlin             |

## Compilation and Execution Commands

| Language  | Compile / Interpreter                                                                                                                                                         | Execution Command        |
|-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------|
| C++17     | `g++ -std=c++17 -Wall -DONLINE_JUDGE=1 -o .qt/main main.cpp`                                                                                                                  | `./.qt/main`             |
| Java      | `javac -d .qt/ Main.java`                                                                                                                                                     | `java -cp .qt/ Main`     |
| Python3   |                                                                                                                                                                               | `python3 main.py`        |
| Rust Lang | `cp main.rs ~/.quicktest/rust/src/main.rs && cargo build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust .qt/main` | `./.qt/main`             |
| Go Lang   | `cp main.go ~/.quicktest/go_mod/main.go && go build -buildmode=exe -o ./.qt/main ~/.quicktest/go_mod/main.go`                                                                 | `./.qt/main`             |
| GNU C     | `gcc -std=gnu11  -lm main.c -o .qt/main`                                                                                                                                      | `./.qt/main`             |
| Kotlin    | `kotlinc main.kt -include-runtime -d .qt/main.jar`                                                                                                                            | `java -jar .qt/main.jar` |

## Configuration File

### Rust Lang

**Path:** `~/.quicktest/rust/Cargo.toml`

```toml
[package]
name = "rust"
version = "0.1.0"
edition = "2021"
[dependencies]
proconio = "0.4.3"
num = "0.4.0"
rand = { version = "0.8.5", features = ["small_rng"]}
regex = "1.5.5"
num-bigint = "0.4.3"
```
