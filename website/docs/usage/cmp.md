---
sidebar_position: 2
title: Cmp subcommand
sidebar_label: Cmp
---

### `quicktest cmp | qt cmp`

Verifies the correctness of an algorithm by comparing it against a brute-force solution. The brute-force solution, while typically slower, is guaranteed to be correct.

```shell
quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp
```

Or using the shorter alias:

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
    * `--prefix=<value> | -p=<value>` Conflicts with `--gen-file` (Only one can be used at a time).
    * `--diff`  Shows differences between the expected file and the output file

* **Flags of the `cmp`, `stress` and `check` subcommands**

    * `--break-bad | --break`  Stops execution if WA, TLE, or RTE states occur.
    * `--run-ac`     Runs Accepted test cases.
    * `--run-all`    Runs all test cases.
    * `--run-rte`    Runs Run Time Error test cases.
    * `--run-tle`    Runs Time Limit Exceeded test cases.
    * `--run-wa`     Runs Wrong Answer test cases.
    * `--save-all`   Saves all test cases.
    * `--save-bad`   Saves only bad cases (WA, TLE, or RTE).