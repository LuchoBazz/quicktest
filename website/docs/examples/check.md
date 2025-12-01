---
sidebar_position: 3
title: Check Examples
sidebar_label: Check
---

## Running Examples

* ### C++ Examples
    ```shell
    git clone https://github.com/LuchoBazz/quicktest.git

    cd quicktest/examples/check/cpp
    ```

    ```shell
    quicktest check --target-file=main.cpp --checker-file=checker.cpp --gen-file=gen.cpp --test-cases=500 --timeout=1000
    ```

    Or using the shorter alias:

    ```shell
    qt check -t main.cpp -c checker.cpp -g gen.cpp --tc 500 --tout 1000
    ```

## Save Test Cases

You can use the following flags:
* `--save-all`   Saves all test cases.
* `--save-bad`   Saves only bad cases (WA, TLE, or RTE).

For example:

```shell
quicktest check --target-file=main.cpp --checker-file=checker.cpp --gen-file=gen.cpp --test-cases=50 --timeout=1000 --save-bad
```

Or using the shorter alias:

```shell
qt check -t main.cpp -c checker.cpp -g gen.cpp --tc 50 --tout 1000 --save-bad
```

## Run Saved Test Cases

You can use the following flags:

* `--run-ac`     Runs Accepted test cases.
* `--run-all`    Runs all test cases.
* `--run-rte`    Runs Run Time Error test cases.
* `--run-tle`    Runs Time Limit Exceeded test cases.
* `--run-wa`     Runs Wrong Answer test cases.

For example:

```shell
quicktest check --target-file=main.cpp --checker-file=checker.cpp --gen-file=gen.cpp --test-cases=50 --timeout=1000 --run-wa
```
Or using the shorter alias:

```shell
qt check -t main.cpp -c checker.cpp -g gen.cpp --tc 50 --tout 1000 --run-wa
```