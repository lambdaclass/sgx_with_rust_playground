This repo contains the example used in the "Exploring Secure computing in the Cloud with SGX in a Rust project" blog post.

To run it you will need to install the fortanix enclave runner. You can check the install instructions and requirements [here](Install-Instructions).
You can otherwise run it as a standard rust binary crate by removing the two lines in `hello-world/enclave/.cargo/config`.

## Install Instructions
The official installation instructions can be found at [fortanix edp documentation](https://github.com/fortanix/rust-sgx#get-started-now), but here are what we ended up doing to set this up.

```bash
# some deps
sudo apt update
sudo apt install gcc pkg-config libssl-dev protobuf-compiler

# intel-sgx-dkms install
echo "deb https://download.fortanix.com/linux/apt xenial main" | sudo tee -a /etc/apt/sources.list.d/fortanix.list >/dev/null
curl -sSL "https://download.fortanix.com/linux/apt/fortanix.gpg" | sudo -E apt-key add -
sudo apt-get update
sudo apt-get install intel-sgx-dkms

# Install EDP components
rustup target add x86_64-fortanix-unknown-sgx --toolchain nightly
cargo install fortanix-sgx-tools sgxs-tools --git https://github.com/fortanix/rust-sgx
echo >> ~/.cargo/config -e '[target.x86_64-fortanix-unknown-sgx]\nrunner = "ftxsgx-runner-cargo"'
```

Notice that we're passing the `--git` argument to `cargo install`. This is because there were problems with nightly after `nightly-2021-12-15` and it has been fixed but the fixed version has not been updated to crates.io yet. Another way to do the install is by:

```bash
rustup default nightly-2021-12-15

# Install EDP components
rustup target add x86_64-fortanix-unknown-sgx --toolchain nightly
cargo install fortanix-sgx-tools sgxs-tools
# The rest is the same
```

For more on this, check the [full issue](https://github.com/fortanix/rust-sgx/issues/374).

Now that you're already set up, try running:
```bash
sgx-detect
```
If you weren't able to install the attestation packages, then you'll see that there are a few libraries missing. This should't be a problem in this case.
```bash
  #...
  ✘  libsgx_enclave_common
  ✘  AESM service
```

```bash
# Run your enclave!
cargo run +nightly --target x86_64-fortanix-unknown-sgx
```

- The `+nightly` option shouldn't be necessary if the default toolchain is already nightly.
- The `--target x86_64-fortanix-unknown-sgx` option shouldn't be necessary because of the lines in `.cargo/config`, but are here in case emphasize that it's not necessary to make the `.cargo` directory
