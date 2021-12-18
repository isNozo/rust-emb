# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.205.1/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends git minicom libusb-1.0.0-dev libsdl2-dev libssl-dev
RUN rustup target add thumbv7em-none-eabihf \
    && cargo install cargo-generate hf2-cli cargo-hf2
