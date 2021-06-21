`quicktest check | qt check`
============

**Verify the correctness of the code using a verifier script:** Similar to the previous one, this test verifies that the code does not have an incorrect answer for some test cases using a verifier script because there may be many correct answers.

## Run Examples

* ### cpp examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/check/cpp

    quicktest check --target-file=main.cpp --checker-file=checker.cpp --gen-file=gen.cpp --test-cases=10 --timeout=1000
    ```

* ### python examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/check/python

    quicktest check --target-file=main.py --checker-file=checker.py --gen-file=gen.py --test-cases=10 --timeout=1000
    ```

