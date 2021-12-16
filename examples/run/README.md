`quicktest run | qt run`
============

## Run Examples

```shell
git clone https://github.com/LuisMBaezCo/quicktest.git

cd quicktest/examples/run
```

* ### cpp examples
   
    ```shell
    quicktest run --target-file=cpp/main.cpp --prefix=test_cases/testcase_ac
    ```
    Or which saves the output to a file
    ```shell
    quicktest run --target-file=cpp/main.cpp --prefix=test_cases/testcase_ac --save-out
    ```

* ### python examples
    
    ```shell
    quicktest run --target-file=python/main.py --prefix=test_cases/testcase_ac
    ```
    Or which saves the output to a file
     ```shell
    quicktest run --target-file=python/main.py --prefix=test_cases/testcase_ac --save-out
    ```
