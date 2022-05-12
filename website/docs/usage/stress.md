---
sidebar_position: 1
title: Stress subcommand
sidebar_label: Stress
---

### `quicktest stress | qt stress`

**Detect cases with TLE:**

Verify that the code execution time does not exceed what is allowed, using a random generator for multiple test cases.

```shell
quicktest stress --target-file=main.cpp --gen-file=gen.cpp
```

or shorter

```shell
qt stress -t main.cpp -g gen.cpp
```

### Subcommand Structure

* `quicktest stress | qt stress`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --tc=<value> [default: 1000]`
    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`

* **Flags of the `cmp`, `stress` and `check` subcommands**

    * `--break-bad | --break`  Break if WA, TLE or RTE states occurs
    * `--run-ac`     Run test cases Accepted
    * `--run-all`    Run all test cases
    * `--run-rte`    Run test cases Run Time Error
    * `--run-tle`    Run test cases Time Limited Exceeded
    * `--run-wa`     Run test cases Wrong Answer
    * `--save-all`   Save all test cases
    * `--save-bad`   Save only bad cases with WA, TLE or RTE states