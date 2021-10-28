# Environment setup

```
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

Install arm compilers

Windows:
1. Install [ARM toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads)
2. Install [xPack OpenOCD](https://xpack.github.io/openocd/install/)
3. Install [ST-Link drivers](https://www.st.com/content/my_st_com/en/products/development-tools/software-development-tools/stm32-software-development-tools/stm32-utilities/stsw-link009.license=1635214882939.product=STSW-LINK009.version=2.0.2.html)


# Useful commands 

```
$ # In ./app or ./bootloader, start openOCD
$ openocd
$ # then debug
$ cargo run

$ # Show sizes
$ cargo size --bin app -- -A

$ # Make bin file
$ cargo objcopy -- -O binary out.bin
```