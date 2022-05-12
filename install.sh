#!/bin/bash

#  This script sets up everything needed to start developing your sgx
#  application using fortanix's EDP. Run this only once. If you want to have
#  all the libs installed, using Ubuntu xenial, focal or bionic is necessary.

###################
#   Dependencies  #
###################
sudo apt update
sudo apt install gcc pkg-config libssl-dev protobuf-compiler

###################
#     sgx-libs    #
###################

# intel-sgx-dkms install
echo "deb https://download.fortanix.com/linux/apt xenial main" | sudo tee -a /etc/apt/sources.list.d/fortanix.list >/dev/null
curl -sSL "https://download.fortanix.com/linux/apt/fortanix.gpg" | sudo -E apt-key add -
sudo apt-get update
sudo apt-get install intel-sgx-dkms

# need xenial, focal or bionic for this to work (more info below)
# Install AESM libs
echo "deb https://download.01.org/intel-sgx/sgx_repo/ubuntu $(lsb_release -cs) main" | sudo tee -a /etc/apt/sources.list.d/intel-sgx.list >/dev/null
curl -sSL "https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key" | sudo -E apt-key add -
sudo apt-get update
sudo apt-get install sgx-aesm-service libsgx-aesm-launch-plugin

###################
#   Rust Install  #
###################

# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update PATH ENV
source ~/.profile

# Use nightly
rustup install toolchain nightly

# Install EDP components
rustup target add x86_64-fortanix-unknown-sgx --toolchain nightly
cargo install fortanix-sgx-tools sgxs-tools --git https://github.com/fortanix/rust-sgx
echo >> ~/.cargo/config -e '[target.x86_64-fortanix-unknown-sgx]\nrunner = "ftxsgx-runner-cargo"'
