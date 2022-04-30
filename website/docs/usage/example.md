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
```

or shorter

```shell
qt example --stress
qt example --cmp
qt example --check
```

### Subcommand Structure

* `quicktest example | qt example`
    
    **Flags**
    
    * `--check`   Show examples of the check subcommand
    * `--cmp`     Show examples of the cmp subcommand
    * `--stress`  Show examples of the stress subcommand
    
    **Nota:** can only use one flag at a time
