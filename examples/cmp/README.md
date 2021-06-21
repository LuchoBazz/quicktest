`quicktest cmp | qt cmp`
============

**Check the correctness of the code compared to a slower version:** Verify that the code does not have incorrect answers for some test cases, using a random generator and a slower version which is sure what is correct with which the answers will be compared.

## Run Examples

* ### cpp examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/cmp/cpp

    quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp --test-cases=10 --timeout=1000
    ```

* ### python examples
    ```shell
    git clone https://github.com/LuisMBaezCo/quicktest.git

    cd quicktest/examples/cmp/python

    quicktest cmp --target-file=main.py --correct-file=correct.py --gen-file=gen.py --test-cases=10 --timeout=1000
    ```

