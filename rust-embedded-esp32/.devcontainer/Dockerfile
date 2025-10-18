FROM ubuntu:latest

# Allow bash in RUN
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Inspiration from:
# - https://github.com/ctron/rust-esp-container/blob/master/Dockerfile
# - https://github.com/esp-rs/no_std-training/blob/main/.devcontainer/Dockerfile
# - https://github.com/esp-rs/esp-idf-template/tree/master/cargo/.devcontainer
RUN apt-get update && apt-get install --no-install-recommends -y \
    build-essential g++ gcc clang make pkgconf pkg-config \
    ca-certificates libssl-dev musl-tools llvm-dev libclang-dev libudev-dev libuv1-dev \
    python3-venv python-is-python3 \
    sudo curl wget git vim \
    rustup \
    minicom esptool lldb \
    && apt-get clean -y
# Add? flex bison gperf cmake ninja-build ccache libffi-dev dfu-util libusb-1.0-0

# Mise
ENV MISE_DATA_DIR="/mise" MISE_CONFIG_DIR="/mise" MISE_CACHE_DIR="/mise/cache" MISE_INSTALL_PATH="/usr/local/bin/mise"
ENV PATH="/mise/shims:~/.cargo/bin:$PATH"
RUN curl https://mise.run | sh

# User
RUN sh -c 'echo "%sudo ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers'
USER ubuntu

# Rust, ESP32 RISC-V targets, tooling
# TODO: add ESP32 XTensa
RUN rustup toolchain install stable --component rust-src
RUN rustup target add riscv32imc-unknown-none-elf  # For ESP32-C2, C3
RUN rustup target add riscv32imac-unknown-none-elf # For ESP32-C6, H2

# Cargo install.
# NOTE: binstall is A LOT faster
RUN cargo install cargo-binstall
RUN cargo binstall espflash cargo-espflash esp-generate cargo-generate probe-rs-tools ldproxy --locked
