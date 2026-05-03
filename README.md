# rust-begin

A collection of bare-metal Rust examples for the nRF54L15 Development Kit.

This repository demonstrates simple embedded applications using Rust on the nRF54L15 DK, including GPIO polling and LED control.

## Build and flash
1. Run `cargo build` to compile the application.
2. Run `cargo flash --chip nRF54L15` to program the board.

## Examples
- `examples/01_blinky`: Blink an LED.
- `examples/02_polling`: Poll a button and blink LED3 while the button is pressed.

## Notes
- The code targets `thumbv8m.main-none-eabihf`.
- The board configuration is defined in `.cargo/config.toml`.
