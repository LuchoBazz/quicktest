---
sidebar_position: 5
title: Setup subcommand
sidebar_label: Setup
---

### `quicktest setup config | qt setup config`


```shell
quicktest setup config --label="Language::Cpp.PROGRAM" --value="g++"
quicktest setup config --label="Language::Cpp.STANDARD" --value="-std=c++17"
quicktest setup config --label="Language::Python.PROGRAM" --value="python"
```

or shorter

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

    * `config` Subcommand that allows to change C++ settings

        **Options**

        * `-l=<value> | --label=<value>` Label with the path of the configuration that you want to change
        * `-v=<value> | --value=<value>` value you want to change a selected label to
