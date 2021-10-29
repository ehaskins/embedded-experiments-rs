#! /usr/bin/env pwsh

$ErrorActionPreference = "Stop"

Push-Location ./bootloader

cargo build --release

Set-Location ../app

cargo build --release
cargo objcopy -- -O binary app.bin

Pop-Location
