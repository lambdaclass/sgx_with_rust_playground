#! /bin/bash

# This requires the user to set appropritate runner in .cargo/config
set -ex
executable='target/x86_64-fortanix-unknown-sgx/debug/hello-world'
cargo clean
cargo  +nightly run --target=x86_64-fortanix-unknown-sgx

echo "Hello World!" | nc 127.0.0.1 7878
