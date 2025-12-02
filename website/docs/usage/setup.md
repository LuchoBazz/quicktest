---
sidebar_position: 5
title: Setup subcommand
sidebar_label: Setup
---

### `quicktest setup config | qt setup config`

To view the complete list of accepted configuration tags, run the following command:

`quicktest setup config --help`

```shell
quicktest setup config --label="Language::Cpp.PROGRAM" --value="g++"
quicktest setup config --label="Language::Cpp.STANDARD" --value="-std=c++17"
quicktest setup config --label="Language::Python.PROGRAM" --value="python"
```

Or using the shorter alias:

```shell
qt setup config -l "Language::Cpp.PROGRAM" -v="g++"
qt setup config -l "Language::Cpp.STANDARD" -v="-std=c++17"
qt setup config -l "Language::Python.PROGRAM" -v="python"
```

### Demo

![setup gif](/gif/setup.gif)

### Subcommand Structure

* `quicktest setup | qt setup`

    **Subcommand**

    * `config` Subcommand for modifying C++ configuration settings.

        **Options**

        * `-l=<value> | --label=<value>` The configuration label path to modify.
        * `-v=<value> | --value=<value>` The new value for the selected label.
