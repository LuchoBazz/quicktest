---
sidebar_position: 4
title: Output Examples
sidebar_label: Output
---

## Running Examples

```shell
git clone https://github.com/LuchoBazz/quicktest.git

cd quicktest/examples/output
```

* ### C++ Examples
   
    ```shell
    quicktest output --target-file=cpp/main.cpp --prefix=test_cases/testcase_ac
    ```
    Or to save the output to a file:
    ```shell
    quicktest output --target-file=cpp/main.cpp --prefix=test_cases/testcase_ac --save-out
    ```

    Or using the shorter alias:

    ```shell
    quicktest output -t cpp/main.cpp -p test_cases/testcase_ac --save-out
    ```

* ### Python Examples
    
    ```shell
    quicktest output --target-file=python/main.py --prefix=test_cases/testcase_ac
    ```
    Or to save the output to a file:
    ```shell
    quicktest output --target-file=python/main.py --prefix=test_cases/testcase_ac --save-out
    ```

    Or using the shorter alias:

    ```shell
    quicktest output -t python/main.py -p test_cases/testcase_ac --save-out
    ```
