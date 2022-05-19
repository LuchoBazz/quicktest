#!/usr/bin/env pwsh
# Quick Test: CLI for stress testing in competitive programming
# Copyright (C) 2021-present / Luis Miguel Báez
# License: MIT (See the LICENSE file in the repository root directory)
# TODO(everyone): Keep this script simple and easily auditable.

if ($v) {
    $Version = "${v}"
} else {
    $Response = Invoke-WebRequest -URI "https://api.github.com/repos/LuisMBaezCo/quicktest/releases/latest" |
        Select-Object -Property Content |
        Select-String -Pattern '"tag_name":"[0-9.]+'
    $Version = $Response.matches.Value.replace('"tag_name":"', '')
}

$QT_Install = $env:QUICK_TEST_INSTALL
$BinDir = if ($QT_Install) {
    "$QT_Install\bin"
} else {
    "$Home\.quicktest\bin"
}

$QT_Zip = "$BinDir\quicktest.zip"
$QuickTestExe = "$BinDir\quicktest.exe"
$QtExe = "$BinDir\qt.exe"
$Target = 'x86_64-pc-windows-msvc'

# GitHub requires TLS 1.2
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$QT_Uri = "https://github.com/LuisMBaezCo/quicktest/releases/latest/download/quicktest-${Target}-v${Version}.zip"

if (!(Test-Path $BinDir)) {
    New-Item $BinDir -ItemType Directory | Out-Null
}

Invoke-WebRequest $QT_Uri -OutFile $QT_Zip -UseBasicParsing

if (Get-Command Expand-Archive -ErrorAction SilentlyContinue) {
    Expand-Archive $QT_Zip -Destination $BinDir -Force
} else {
    if (Test-Path $QuickTestExe) {
        Remove-Item $QuickTestExe
    }
    if (Test-Path $QtExe) {
        Remove-Item $QtExe
    }
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [IO.Compression.ZipFile]::ExtractToDirectory($QT_Zip, $BinDir)
}

Remove-Item $QT_Zip

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable('Path', $User)
if (!(";$Path;".ToLower() -like "*;$BinDir;*".ToLower())) {
    [Environment]::SetEnvironmentVariable('Path', "$Path;$BinDir", $User)
    $Env:Path += ";$BinDir"
}

Write-Output "Quick Test CLI - ${Version} was installed successfully to $QuickTestExe"
Write-Output "Run 'quicktest --help' or 'qt --help' to get started"
