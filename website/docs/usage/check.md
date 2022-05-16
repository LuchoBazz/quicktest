---
sidebar_position: 3
title: Check subcommand
sidebar_label: Check
---

### `quicktest check | qt check`

In some problems more than one answer is accepted, so the `quicktest cmp` command would not work correctly, in this case a script checker is used to verify the correctness of the algorithm.

```shell
quicktest check --target-file=main.cpp --checker-file=correct.cpp --gen-file=gen.cpp
```

or shorter

```shell
qt check --t main.cpp -c correct.cpp -g gen.cpp
```

### Demo

![check gif](/gif/check.gif)

### Subcommand Structure

* `quicktest check | qt check`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-c=<value> | --checker-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --tc=<value> [default: 1000]`
    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--prefix=<value> | -p=<value>` conflict with `--gen-file` (Only one can be used at a time)1

* **Flags of the `cmp`, `stress` and `check` subcommands**

    * `--break-bad | --break`  Break if WA, TLE or RTE states occurs
    * `--run-ac`     Run test cases Accepted
    * `--run-all`    Run all test cases
    * `--run-rte`    Run test cases Run Time Error
    * `--run-tle`    Run test cases Time Limited Exceeded
    * `--run-wa`     Run test cases Wrong Answer
    * `--save-all`   Save all test cases
    * `--save-bad`   Save only bad cases with WA, TLE or RTE states