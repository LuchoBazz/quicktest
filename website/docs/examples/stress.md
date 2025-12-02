---
sidebar_position: 1
title: Stress Examples
sidebar_label: Stress
---

## Running Examples

* ### C++ Examples
    ```shell
    git clone https://github.com/LuchoBazz/quicktest.git

    cd quicktest/examples/stress/cpp
    ```

    ```shell
    quicktest stress --target-file=main.cpp --gen-file=gen.cpp --test-cases=15 --timeout=1000
    ```

    Or using the shorter alias:

    ```shell
    quicktest stress -t main.cpp -g gen.cpp --tc 15 --tout 1000
    ```


* ### Python Examples
    ```shell
    git clone https://github.com/LuchoBazz/quicktest.git

    cd quicktest/examples/stress/python
    ```

    ```shell
    quicktest stress --target-file=main.py --gen-file=gen.py --test-cases=15 --timeout=1000
    ```

    Or using the shorter alias:

    ```shell
    quicktest stress -t main.py -t gen.py --tc 15 --tout 1000
    ```


## Save Test Cases

You can use the following flags:
* `--save-all`   Saves all test cases.
* `--save-bad`   Saves only bad cases (WA, TLE, or RTE).

For example:

```shell
quicktest stress --target-file=main.cpp --gen-file=gen.cpp --test-cases=15 --timeout=1000 --save-bad
```

Or using the shorter alias:

```shell
quicktest stress --t main.cpp -g gen.cpp --tc 15 --tout 1000 --save-bad
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
quicktest stress --target-file=main.cpp --gen-file=gen.cpp --test-cases=15 --timeout=1000 --run-tle
```

Or using the shorter alias:

```shell
quicktest stress -t main.cpp -g gen.cpp --tc 15 --tout 1000 --run-tle
```