Push-Location ./bootloader

cargo build

Set-Location ../app

cargo build
cargo objcopy -- -O binary app.bin

Pop-Location
