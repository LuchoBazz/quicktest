---
sidebar_position: 2
title: Cmp Examples
sidebar_label: Cmp
---

## Running Examples

* ### C++ Examples
    ```shell
    git clone https://github.com/LuchoBazz/quicktest.git

    cd quicktest/examples/cmp/cpp
    ```

    ```shell
    quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp --test-cases=500 --timeout=1000
    ```

    Or using the shorter alias:

    ```shell
    qt cmp -t main.cpp -c correct.cpp -g gen.cpp --tc 500 --tout 1000
    ```

* ### Python Examples
    ```shell
    git clone https://github.com/LuchoBazz/quicktest.git

    cd quicktest/examples/cmp/python
    ```
    
    ```shell
    quicktest cmp --target-file=main.py --correct-file=correct.py --gen-file=gen.py --test-cases=200 --timeout=1000
    ```

    Or using the shorter alias:

    ```shell
    qt cmp -t main.py -c correct.py -g gen.py --tc 200 --tc 1000
    ```

## Save Test Cases

You can use the following flags:
* `--save-all`   Saves all test cases.
* `--save-bad`   Saves only bad cases (WA, TLE, or RTE).

For example:

```shell
quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp --test-cases=500 --timeout=1000 --save-bad
```

or shorter

```shell
quicktest cmp -t main.cpp -c correct.cpp -g gen.cpp --tc 500 --tout 1000 --save-bad
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
quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp --test-cases=500 --timeout=1000 --run-wa
```

or shorter

```shell
quicktest cmp -t main.cpp -c correct.cpp -g gen.cpp --tc 500 --tout 1000 --run-wa
```