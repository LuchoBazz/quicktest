---
sidebar_position: 6
title: Example subcommand
sidebar_label: Example
---

### `quicktest example | qt example`

```shell
quicktest example --stress
quicktest example --cmp
quicktest example --check
quicktest example --run
quicktest example --setup
```

Or using the shorter alias:

```shell
qt example --stress
qt example --cmp
qt example --check
qt example --run
qt example --setup
```

### Demo

![example gif](/gif/example.gif)

### Subcommand Structure

* `quicktest example | qt example`
    
    **Flags**
    
    * `--check`   Shows examples for the check subcommand.
    * `--cmp`     Shows examples for the cmp subcommand.
    * `--stress`  Shows examples for the stress subcommand.
    * `--run`     Shows examples for the run subcommand.
    * `--setup`   Shows examples for the setup subcommand.
    
    **Note:** Only one flag can be used at a time.
