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

or shorter

```shell
qt example --stress
qt example --cmp
qt example --check
qt example --run
qt example --setup
```

### Subcommand Structure

* `quicktest example | qt example`
    
    **Flags**
    
    * `--check`   Show examples of the check subcommand
    * `--cmp`     Show examples of the cmp subcommand
    * `--stress`  Show examples of the stress subcommand
    * `--run`     Show examples of the run subcommand
    * `--setup`   Show examples of the setup subcommand
    
    **Nota:** can only use one flag at a time
