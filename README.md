#  🐳 `epd24rwb`

 **📦  Linux epaper display driver writed [🦀 **Rust**](https://github.com/smolkov/automata)**
 ** Hardware [💵 **inky-phat**](https://shop.pimoroni.com/products/inky-phat) **

🚧 _Work In Progress_ 🚧

[![travis build Status](https://travis-ci.com/lar-rs/edinburgh.svg?branch=master)](https://travis-ci.com/smolkov/automata)
[![builds.sr.ht status](https://builds.sr.ht/~asmolkov/wqa/.build.yml.svg)](https://builds.sr.ht/~asmolkov/automata/.build.yml?)
[![open issue]][issue]
![Minimum Rust Version][min-rust-badge]

## 🎙️ Commands

`epd24rbw` is a CLI tool designed for setup and read ndir sensors data.

  - `address`: defaults name `system`. For remote acces use adress like this `host=0.0.0.0,port=6666`

  - ### 🦀⚙️ `driver`
    run driver and bind directory to wath data.
    All of the arguments and flags to this command are optional:
        - `path`: working directory default to`/var/run/greenhouse/dashboard/epd24rwb`

  - ### 🔧 `setup`
        - `bautrate`: default `57600`
        - `uart`:  defaults to `1` and iterate in `1..4`
      Configure display for user.

    ```
    epd24rwb config --uart=2 --path=`test1`
    ```
## Configuration

`init` initialize directory.



## 🔩 Building

To cross-compile for the Raspberry Pi you will need an
`arm-unknown-linux-gnueabihf` GCC toolchain and Rust component installed. On
Arch Linux I built [arm-linux-gnueabihf-gcc] from the AUR. Add the Rust target
with `rustup target add arm-unknown-linux-gnueabihf`. Then you can
cross-compile with `cargo`:

    cargo build --release --target arm-unknown-linux-gnueabihf

After it is built copy `target/arm-unknown-linux-gnueabihf/release/lca2019` to
the Raspberry Pi.



## ⚓ Installation

1. Install `cargo`:

    Edinburgh is installed through [Cargo](https://github.com/rust-lang/cargo#compiling-from-source), a Rust package manager. Rustup, a tool for installing Rust, will also install Cargo. On Linux and macOS systems, `rustup` can be installed as follows:

    ```
    curl https://sh.rustup.rs -sSf | sh
    ```

    Additional installation methods are available [here](https://forge.rust-lang.org/other-installation-methods.html).

2. Install `automata`:

    ```
    cargo install automata
    ```

- **Troubleshooting** `libdbus-sys` errors

    On Ubuntu OS install

    ```
    $ sudo apt install libdbus-1-dev
    ```

### 🔬 🕵️‍♀️ systemd service

Copy `epd24rwb.service` to `/etc/systemd/system/`.

    sudo systemctl daemon-reload
    sudo setcap cap_net_bind_service=epd24rwb
    sudo systemctl enable --now epd24rwb

<!-- Badges -->
[issue]: https://img.shields.io/github/issues/greenhouse/epd24rwb?style=flat-square
[min-rust-badge]: https://img.shields.io/badge/rustc-1.38+-blue.svg

<!-- Server on tide [creating 🌊 web-server .deb binary with rust](https://gi.net.in/posts/creating-web-server-deb-binary-with-rust/) -->

