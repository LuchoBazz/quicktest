---
sidebar_position: 3
title: Check subcommand
sidebar_label: Check
---

### `quicktest check | qt check`

For problems where multiple answers are valid, `quicktest cmp` may not work correctly. In these cases, the `check` command uses a custom checker script to verify the correctness of your algorithm's output.

```shell
quicktest check --target-file=main.cpp --checker-file=correct.cpp --gen-file=gen.cpp
```

Or using the shorter alias:

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
    * `--prefix=<value> | -p=<value>` Conflicts with `--gen-file` (Only one can be used at a time).

* **Flags of the `cmp`, `stress` and `check` subcommands**

    * `--break-bad | --break`  Stops execution if WA, TLE, or RTE states occur.
    * `--run-ac`     Runs Accepted test cases.
    * `--run-all`    Runs all test cases.
    * `--run-rte`    Runs Run Time Error test cases.
    * `--run-tle`    Runs Time Limit Exceeded test cases.
    * `--run-wa`     Runs Wrong Answer test cases.
    * `--save-all`   Saves all test cases.
    * `--save-bad`   Saves only bad cases (WA, TLE, or RTE).