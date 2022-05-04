---
sidebar_position: 4
title: Run subcommand
sidebar_label: Run
---

### `quicktest run | qt run`

**Run a target file with test case files matching a prefix:**

```shell
quicktest run --target-file=main.cpp --prefix=testcase_ac
```

or shorter

```shell
qt run -t main.cpp -p testcase_ac
```

### Subcommand Structure

* `quicktest run | qt run`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-p=<value> | --prefix=<value>`

    **Other Options**

    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--break-bad | --break`  Break if WA, TLE or RTE states occurs
    * `--save-out`   Save the output of the target file for each test case
