`quicktest output | qt output`
============

## Run Examples

```shell
git clone https://github.com/LuchoBazz/quicktest.git

cd quicktest/examples/output
```

* ### cpp examples
   
    ```shell
    quicktest output --target-file=cpp/main.cpp --prefix=test_cases/testcase_ac
    ```
    Or which saves the output to a file
    ```shell
    quicktest output --target-file=cpp/main.cpp --prefix=test_cases/testcase_ac --save-out
    ```

    or shorter

    ```shell
    quicktest output -t cpp/main.cpp -p test_cases/testcase_ac --save-out
    ```

* ### python examples
    
    ```shell
    quicktest output --target-file=python/main.py --prefix=test_cases/testcase_ac
    ```
    Or which saves the output to a file
    ```shell
    quicktest output --target-file=python/main.py --prefix=test_cases/testcase_ac --save-out
    ```

    or shorter

    ```shell
    quicktest output -t python/main.py -p test_cases/testcase_ac --save-out
    ```
