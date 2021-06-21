`quicktest tle | qt tle`
============

**Detect cases with TLE:** Verify that the code execution time does not exceed what is allowed, using a random generator for multiple test cases.
    * **Sample:**
        ```shell
        quicktest tle --target-file=main.cpp --gen-file=gen.cpp
        ```

## Run Examples

* ### cpp examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/tle/cpp

    quicktest tle --target-file=main.cpp --gen-file=gen.cpp --test-cases=10 --timeout=1000
    ```

* ### python examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/tle/python

    quicktest tle --target-file=main.py --gen-file=gen.py --test-cases=10 --timeout=1000
    ```

