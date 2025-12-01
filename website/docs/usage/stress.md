---
sidebar_position: 1
title: Stress subcommand
sidebar_label: Stress
---

### `quicktest stress | qt stress`

Verifies that the code executes within the time limit using a random generator for multiple test cases.

**Note:** This mode does not compare against a slower, correct solution.

```shell
quicktest stress --target-file=main.cpp --gen-file=gen.cpp
```

Or using the shorter alias:

```shell
qt stress -t main.cpp -g gen.cpp
```

### Demo

![stress gif](/gif/stress.gif)

### Subcommand Structure

* `quicktest stress | qt stress`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
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