# BBC MicroBit v2.0 - Rust Bare Metal

## Intro

I am mainly following the advice found from these two sources:

1. [micro::bit v2 Embedded Discovery Book](https://docs.rust-embedded.org/discovery-mb2/)
2. [The Rusty Bits on YT](https://www.youtube.com/@therustybits)

This is a very basic setup for running Rust on the bbc microbit v2.0 in VSCode.

## VSCode Extensions

VSCode Extensions must-have:

[Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

[Debugger for probe-rs](https://marketplace.visualstudio.com/items?itemName=probe-rs.probe-rs-debugger)

VSCode Extensions nice-to-have:

[Dependi](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi) <- former crates extension

[Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)

[Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)

## Load target

`rustup target add thumbv7em-none-eabihf`

You should now see it listed in here:
`rustup target list`

## Build

`cargo check`
`cargo build`

## Check

Let's see if we actually built an arm binary
`cargo readobj -- --file-headers`

## Flash it

Let's flash our compiled binary to the microbit
`cargo embed`

- Debug GDB -> or use probe-rs setup in launch.json for VSCode
  
`arm-none-eabi-gdb ./target/thumbv7em-none-eabihf/debug/micro`
`target remote :1337` <-Important!!
`monitor reset halt`
`break main`
`info break`
`delete <breakpoint-num>`

## Debug in VScode

Use the [launch.json](.vscode/launch.json) setup for probe-rs (extension must be installed)

## Debug GDB  

1. `arm-none-eabi-gdb ../../../target/thumbv7em-none-eabihf/debug/examples/init`
2. `target remote :1337` -> Connect to our target
3. `monitor reset halt`
4. `break main`
5. `info break`

further more
`delete <breakpoint-num>`
`layout src`
`tui disable`
