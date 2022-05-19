#!/bin/sh
# Quick Test: CLI for stress testing in competitive programming
# Copyright (C) 2021-present / Luis Miguel Báez
# License: MIT (See the LICENSE file in the repository root directory)
# TODO(everyone): Keep this script simple and easily auditable.

set -e

if ! command -v unzip >/dev/null; then
    echo "Error: unzip is required to install Quick Test CLI (see: https://luismbaezco.github.io/quicktest/docs/getting-started/installation#unzip-is-required)." 1>&2
    exit 1
fi

if [ "$OS" = "Windows_NT" ]; then
    target="x86_64-pc-windows-msvc"
else
    target="x86_64-unknown-linux-gnu" ;
fi

if [ "$v" = "" ]; then
    version=$(curl -s "https://api.github.com/repos/LuisMBaezCo/quicktest/releases/latest" | grep -Po '"tag_name": "\K[0-9.]+')
else
    version="$v";
fi

qt_uri="https://github.com/LuisMBaezCo/quicktest/releases/latest/download/quicktest-${target}-v${version}.zip"

quicktest_install="${QT_INSTALL:-$HOME/.quicktest}"
bin_dir="$quicktest_install/bin"
quicktest_exe="$bin_dir/quicktest"
qt_exe="$bin_dir/qt"

if [ ! -d "$bin_dir" ]; then
    mkdir -p "$bin_dir"
fi

curl --fail --location --progress-bar --output "$quicktest_exe.zip" "$qt_uri"
unzip -d "$bin_dir" -o "$quicktest_exe.zip"
chmod +x "$quicktest_exe"
chmod +x "$qt_exe"
rm "$quicktest_exe.zip"

echo "Quick Test CLI was installed successfully to $quicktest_exe"
if command -v quicktest >/dev/null; then
    echo "Run 'quicktest --help' or 'qt --help' to get started"
else
    case $SHELL in
        /bin/zsh) shell_profile=".zshrc" ;;
        *) shell_profile=".bashrc" ;;
    esac

    echo "\nexport QT_INSTALL=\"$quicktest_install\"" >> "$HOME/$shell_profile"
    echo "\nexport PATH=\"\$QT_INSTALL/bin:\$PATH\"\n" >> "$HOME/$shell_profile"
    
    echo "\nManually run the following command 'source $HOME/$shell_profile'"
    echo "Run 'quicktest --help' to get started"
fi
