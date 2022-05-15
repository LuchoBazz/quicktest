---
sidebar_position: 4
title: Output subcommand
sidebar_label: Output
---

### `quicktest output | qt output`

**Run a target file with test case files matching a prefix:**

```shell
quicktest output --target-file=main.cpp --prefix=testcase_ac
```

or shorter

```shell
qt output -t main.cpp -p testcase_ac
```

### Demo

![output gif](/gif/output.gif)

### Subcommand Structure

* `quicktest output | qt output`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-p=<value> | --prefix=<value>`

    **Other Options**

    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--break-bad | --break`  Break if WA, TLE or RTE states occurs
    * `--save-out`   Save the output of the target file for each test case
