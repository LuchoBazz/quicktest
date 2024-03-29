---
sidebar_position: 2
title: Cmp subcommand
sidebar_label: Cmp
---

### `quicktest cmp | qt cmp`

It checks the correctness of the algorithm we want to verify by comparing it with a brute force solution which is usually very slow, but is 100% sure to provide the correct solution.

```shell
quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp
```

or shorter

```shell
qt cmp -t main.cpp --c correct.cpp -g gen.cpp
```

### Demo

![cmp gif](/gif/cmp.gif)

### Subcommand Structure

* `quicktest cmp | qt cmp`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-c=<value> | --correct-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --tc=<value> [default: 1000]`
    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--prefix=<value> | -p=<value>` conflict with `--gen-file` (Only one can be used at a time)
    * `--diff`  Show differences between the expected file and the output file

* **Flags of the `cmp`, `stress` and `check` subcommands**

    * `--break-bad | --break`  Break if WA, TLE or RTE states occurs
    * `--run-ac`     Run test cases Accepted
    * `--run-all`    Run all test cases
    * `--run-rte`    Run test cases Run Time Error
    * `--run-tle`    Run test cases Time Limited Exceeded
    * `--run-wa`     Run test cases Wrong Answer
    * `--save-all`   Save all test cases
    * `--save-bad`   Save only bad cases with WA, TLE or RTE states