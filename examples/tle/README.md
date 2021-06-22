`quicktest tle | qt tle`
============

**Detect cases with TLE:** Verify that the code execution time does not exceed what is allowed, using a random generator for multiple test cases.

## Run Examples

* ### cpp examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/tle/cpp
    ```

    ```shell
    quicktest tle --target-file=main.cpp --gen-file=gen.cpp --test-cases=15 --timeout=1000
    ```

* ### python examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/tle/python
    ```

    ```shell
    quicktest tle --target-file=main.py --gen-file=gen.py --test-cases=15 --timeout=1000
    ```

## Save Test Cases

you can use the following flags
* `--save-all`   Save all test cases
* `--save-bad`   Save only bad cases with WA, TLE or RTE states

for example

```shell
quicktest tle --target-file=main.cpp --gen-file=gen.cpp --test-cases=15 --timeout=1000 --save-bad
```

## Run Saved Test Cases

you can use the following flags

* `--run-ac`     Run test cases Accepted
* `--run-all`    Run all test cases
* `--run-rte`    Run test cases Run Time Error
* `--run-tle`    Run test cases Time Limited Exceeded
* `--run-wa`     Run test cases Wrong Answer

for example

```shell
quicktest tle --target-file=main.cpp --gen-file=gen.cpp --test-cases=15 --timeout=1000 --run-tle
```