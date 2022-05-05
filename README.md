This repo contains the example used in the "Exploring Secure computing in the Cloud with SGX in a Rust project" blog post.

To run it you will need to install the fortanix enclave runner. You can check the install instructions and requirements [here](https://github.com/fortanix/rust-sgx#get-started-now).
You can otherwise run it as a standard rust binary crate by removing the two lines in `hello-world/enclave/.cargo/config`.
