`quicktest cmp | qt cmp`
============

**Check the correctness of the code compared to a slower version:** Verify that the code does not have incorrect answers for some test cases, using a random generator and a slower version which is sure what is correct with which the answers will be compared.

## Run Examples

* ### cpp examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/cmp/cpp
    ```

    ```shell
    quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp --test-cases=500 --timeout=1000
    ```

    or shorter

    ```shell
    qt cmp -t main.cpp -c correct.cpp -g gen.cpp -tc 500 --tout 1000
    ```

* ### python examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/cmp/python
    ```
    
    ```shell
    quicktest cmp --target-file=main.py --correct-file=correct.py --gen-file=gen.py --test-cases=200 --timeout=1000
    ```

    or shorter

    ```shell
    qt cmp -t main.py -c correct.py -g gen.py --tc 200 --tc 1000
    ```

## Save Test Cases

you can use the following flags
* `--save-all`   Save all test cases
* `--save-bad`   Save only bad cases with WA, TLE or RTE states

for example

```shell
quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp --test-cases=500 --timeout=1000 --save-bad
```

or shorter

```shell
quicktest cmp -t main.cpp -c correct.cpp -g gen.cpp --tc 500 --tout 1000 --save-bad
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
quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp --test-cases=500 --timeout=1000 --run-wa
```

or shorter

```shell
quicktest cmp -t main.cpp -c correct.cpp -g gen.cpp --tc 500 --tout 1000 --run-wa
```