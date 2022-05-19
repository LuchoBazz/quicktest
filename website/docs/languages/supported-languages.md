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

| Language  | Compile / Interpreter                                                                                                                                                     | Execution Command           |
|-----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------|
| C++17     | `g++ -std=c++17 -Wall -DONLINE_JUDGE=1 -o main main.cpp`                                                                                                                  | `./main`                    |
| Java      | `javac -d .qt/ Main.java`                                                                                                                                              | `java -cp .qt/ Main`     |
| Python3   |                                                                                                                                                                           | `python3 main.py`           |
| Rust Lang | `cp main.rs ~/.quicktest/rust/src/main.rs && cargo build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust main` | `./main`                    |
| Go Lang   | `cp main.go ~/.quicktest/go_mod/main.go && go build -buildmode=exe -o ./main ~/.quicktest/go_mod/main.go`                                                          | `./main`                    |
| GNU C     | `gcc -std=gnu11 main.c -o main`                                                                                                                                    | `./main`                    |
| Kotlin    | `kotlinc main.kt -include-runtime -d .qt/main.jar`                                                                                                                     | `java -jar .qt/main.jar` |
