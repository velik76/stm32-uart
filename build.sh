#!/bin/sh

cargo build --release # --verbose
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/stm32f429 stm32f429-disco_blink.bin
