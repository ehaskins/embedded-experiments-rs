Push-Location ./bootloader

cargo build

Set-Location ../app

cargo build --features application

Pop-Location
