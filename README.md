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

# need xenial, focal or bionic for this to work (more info below)
# Install AESM libs
echo "deb https://download.01.org/intel-sgx/sgx_repo/ubuntu $(lsb_release -cs) main" | sudo tee -a /etc/apt/sources.list.d/intel-sgx.list >/dev/null
curl -sSL "https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key" | sudo -E apt-key add -
sudo apt-get update
sudo apt-get install sgx-aesm-service libsgx-aesm-launch-plugin

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

There's another thing to install, which is the AESM service. There are only libraries for Ubuntu 16, Ubuntu 18 and Ubuntu 20, so beware if you have a more recent version, you probably won't be able to install this service. You can check your version using:

```bash
# in case you don't have lsb-release installed
# sudo apt update && sudo apt install lsb-release
lsb_release -cs 
```

Supported versions are `xenial`, `focal` and `bionic`. In case you have one of those, try running:
```bash
echo "deb https://download.01.org/intel-sgx/sgx_repo/ubuntu $(lsb_release -cs) main" | sudo tee -a /etc/apt/sources.list.d/intel-sgx.list >/dev/null
curl -sSL "https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key" | sudo -E apt-key add -
sudo apt-get update
sudo apt-get install sgx-aesm-service libsgx-aesm-launch-plugin
```

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

The `+nightly` option shouldn't be necessary if the default toolchain is already nightly.
