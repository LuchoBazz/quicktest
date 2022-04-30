---
sidebar_position: 2
title: Cmp subcommand
sidebar_label: Cmp
---

### `quicktest cmp | qt cmp`

**Check the correctness of the code compared to a slower version:** 

Verify that the code does not have incorrect answers for some test cases, using a random generator and a slower version which is sure what is correct with which the answers will be compared.

```shell
quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp
```

or shorter

```shell
qt cmp -t main.cpp --c correct.cpp -g gen.cpp
```

### Subcommand Structure

* `quicktest cmp | qt cmp`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-c=<value> | --correct-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --cases=<value> [default: 1000]`
    * `--timeout=<value>  [default: 2000]` Unit of time: `ms`
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