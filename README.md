# Game_002_r — Bevy game

This repository contains a minimal Bevy (Rust) application.

Prerequisites
- Rust toolchain (rustc + cargo). Install via rustup: https://rustup.rs
- System packages needed to compile Bevy and native dependencies (Linux / Ubuntu-like):

Minimal (fixes the immediate pkg-config/libudev issue):

```bash
sudo apt update
sudo apt install -y pkg-config libudev-dev
```

Recommended (covers audio, windowing and other native headers used by Bevy):

```bash
sudo apt update
sudo apt install -y pkg-config build-essential libasound2-dev libudev-dev \
  libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev \
  libdbus-1-dev libwayland-dev libxkbcommon-dev libegl1-mesa-dev \
  libx11-xcb-dev libxcb1-dev libxcb-randr0-dev libxcb-xfixes0-dev \
  libxcb-shape0-dev libxcb-xinerama0-dev libxcb-keysyms1-dev
```

If you prefer a minimal quick test and you're on another distribution, consult your package manager for `pkg-config`, `libudev-dev` (or `libsystemd-dev`) and `libasound2-dev` / `alsa-lib-devel`.

Build and run

```bash
# build
cargo build

# run
cargo run
```

Verification

To verify pkg-config finds libudev:

```bash
pkg-config --libs --cflags libudev
```

If the command prints flags (e.g. `-I... -L... -ludev`) you're good. If it errors, search for `libudev.pc` and set PKG_CONFIG_PATH accordingly.

Notes
- If you don't need audio or some features, it may be possible to disable certain features in `Cargo.toml`, but installing the native dev packages is usually easiest.# Game_002_r