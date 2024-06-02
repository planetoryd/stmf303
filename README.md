
# Devices that run on my STM32F3 chip

> initialized with `cargo pio new -b genericSTM32F303CB -f cmsis -p ststm32 -t arm-none-eabi stmf3`

```sh
source .env
# to start receiving logs
probe-rs attach .pio/build/debug/firmware.elf 
```