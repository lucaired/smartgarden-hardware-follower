# Smartgarden Hardware Follower
[![Rust](https://github.com/lucaired/smartgarden-hardware-follower/actions/workflows/rust.yml/badge.svg)](https://github.com/lucaired/smartgarden-hardware-follower/actions/workflows/rust.yml)

You will need:
- A Raspberry Pi with USB port and networking capabilities, i'm using a model 3 B V1.2, but feel free to check out the other [Pis](https://www.raspberrypi.org/products/).
- A USB controlled fan, I'm using the [Breeze-Mobile](https://www.arctic.de/en/Breeze-Mobile/ABACO-BZG00-01000).

## Setup Rust on a Raspberry Pi

Download the installer:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Select `2) Custom installation`, for `Default host triple?` enter `arm-unknown-linux-gnueabihf` and the `nightly` toolchain.

Since `rocket` requires *nightly*, we need to set this.

## Install USB control library

I'm using the [uhubctl](https://github.com/mvp/uhubctl) here, install with
```
sudo apt-get install uhubctl
```

## Start the fan service
- `cargo build`
- `sudo ./target/debug/smartgarden-hardware-follower`

## Future Work
- [ ] user space usb control software
