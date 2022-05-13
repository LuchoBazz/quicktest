`quicktest check | qt check`
============

**Verify the correctness of the code using a verifier script:** Similar to the previous one, this test verifies that the code does not have an incorrect answer for some test cases using a verifier script because there may be many correct answers.

## Run Examples

* ### cpp examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/check/cpp
    ```

    ```shell
    quicktest check --target-file=main.cpp --checker-file=checker.cpp --gen-file=gen.cpp --test-cases=500 --timeout=1000
    ```

    or shorter

    ```shell
    qt check -t main.cpp -c checker.cpp -g gen.cpp --tc 500 --tout 1000
    ```

## Save Test Cases

you can use the following flags
* `--save-all`   Save all test cases
* `--save-bad`   Save only bad cases with WA, TLE or RTE states

for example

```shell
quicktest check --target-file=main.cpp --checker-file=checker.cpp --gen-file=gen.cpp --test-cases=50 --timeout=1000 --save-bad
```

or shorter

```shell
qt check -t main.cpp -c checker.cpp -g gen.cpp --tc 50 --tout 1000 --save-bad
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
quicktest check --target-file=main.cpp --checker-file=checker.cpp --gen-file=gen.cpp --test-cases=50 --timeout=1000 --run-wa
```
or shorter

```shell
qt check -t main.cpp -c checker.cpp -g gen.cpp --tc 50 --tout 1000 --run-wa
```