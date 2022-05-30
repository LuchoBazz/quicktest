---
sidebar_position: 1
title: Installation
sidebar_label: Installation
---

Quick Test CLI works on Linux, Windows, and macOS. Quick Test CLI is a single binary executable. It has no external dependencies.

[quicktest_install](https://github.com/LuisMBaezCo/quicktest/tree/main/website/static/install) provides convenience scripts to download and install the binary.

### Linux

Using Shell:

```shell
curl -fsSL https://luismbaezco.github.io/quicktest/install/install.sh | sh
```

Please open a new terminal, or run the following in the existing one `source ~/.bashrc` or `source ~/.zshrc` as appropriate

### Windows

Using PowerShell:

```powershell
iwr https://luismbaezco.github.io/quicktest/install/install.ps1 -useb | iex
```

### macOS, Linux or Windows
If you already have Rust on your system:

```sh
cargo install quicktest
```

If you don't have rust installed on your system, the following command will install Rust and the CLI at once:

### macOS or Linux

Shell:

```sh
curl https://sh.rustup.rs -sSf | sh  && cargo install quicktest
```
## Known Issues

### unzip is required

The program [`unzip`](https://linux.die.net/man/1/unzip) is a requirement for the Shell installer.

```sh
$ curl -fsSL https://luismbaezco.github.io/quicktest/install/install.sh | sh
Error: unzip is required to install Quick Test CLI (see: https://luismbaezco.github.io/quicktest/docs/getting-started/installation#unzip-is-required).
```

**When does this issue occur?**

During the `install.sh` process, `unzip` is used to extract the zip archive.

**How can this issue be fixed?**

You can install unzip via `brew install unzip` on MacOS or `apt-get install unzip -y` on Linux.