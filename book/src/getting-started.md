# Getting Started

## Installing The Requirements

To use QCGPU, you will need Rust and OpenCL installed.

The setup process for OpenCL will be different for every device. All apple devices (MacOS / OSX) will have OpenCL already installed.
For some hints on how to install on linux, look at the [AWS EC2 Install Instructions](https://github.com/QCGPU/qcgpu-rust/blob/master/EC2-install.md).
There is also a good chance that it is installed already. Check that `clinfo` for some other diagnostic command will run.

Rust is very easy to install. Check out [rustup](https://www.rustup.rs).

## Adding The Dependency

To use the library with rust, you must add the following to your `cargo.toml` file:

```toml
[dependencies]
qcgpu = "0.1"
```

You should now be able to use the library by adding 

```
extern crate qcgpu;
```

to `lib.rs` or `main.rs`, depending on if you are writing an executable or a library.
