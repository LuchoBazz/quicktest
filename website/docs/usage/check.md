---
sidebar_position: 3
title: Check subcommand
sidebar_label: Check
---

### `quicktest check | qt check`

**Verify the correctness of the code using a verifier script:**

Similar to the previous one, this test verifies that the code does not have an incorrect answer for some test cases using a verifier script because there may be many correct answers.

```shell
quicktest check --target-file=main.cpp --checker-file=correct.cpp --gen-file=gen.cpp
```

or shorter

```shell
qt check --t main.cpp -c correct.cpp -g gen.cpp
```

### Subcommand Structure

* `quicktest check | qt check`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-c=<value> | --checker-file=<value>`
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