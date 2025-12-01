---
sidebar_position: 4
title: Output subcommand
sidebar_label: Output
---

### `quicktest output | qt output`

Runs all test cases matching a prefix and saves the results to an output file.

```shell
quicktest output --target-file=main.cpp --prefix=testcase_ac
```

Or using the shorter alias:

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
    * `--break-bad | --break`  Stops execution if WA, TLE, or RTE states occur.
    * `--save-out`   Saves the target file output for each test case.
