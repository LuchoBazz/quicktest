`quicktest setup | qt setup`
============

## C++
  
* ### Change the `program` argument of the compilation command in C++
    `[`**default:** `g++` `]`

    ```shell
    quicktest setup cpp --program="g++"
    ```

* ### Change the `standard` argument of the compilation command in C+
    `[`**default:** `-std=c++17` `]`

    ```shell
    quicktest setup cpp --standard="-std=c++20"
    ```

* ### Change the `flags` argument of the compilation command in C+
    `[`**default:** `-Wall -DONLINE_JUDGE=1` `]`

    ```shell
    quicktest setup cpp --flags="-Wall;-DONLINE_JUDGE=1"
    ```

## Python
  
* ### Change the `program` argument of the run command in Python
    `[`**default:** `python` `]`

    ```shell
    quicktest setup python --program="python3"
    ```

* ### Change run command `flags` arguments in Python
    `[`**default:** `ONLINE_JUDGE=1` `]`

    ```shell
    quicktest setup cpp --flags="ONLINE_JUDGE=1;LOCAL=1" `]`
    ```
