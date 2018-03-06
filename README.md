# QCGPU

> An Open Source, High Performance & GPU Accelerated, Quantum Computer Simulator in Rust

* Uses the parallel programing library [ArrayFire](http://arrayfire.org/docs/index.htm), which supports CUDA and OpenCL kernels.
* Runs Cross Platform (x86, ARM, CUDA, and OpenCL devices)

## Prerequisites:
* ArrayFire 3.5.x or newer via. [pre-built binaries](http://arrayfire.com/download)
* Set the environment variable AF_PATH to point to the ArrayFire installation root folder
* Add the library files to your PATH
    * Windows: Add `%AF_PATH%\lib` to your PATH
    * MacOS / Linux: run `export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$AF_PATH/lib`
* Rust (install [here](https://www.rustup.rs)). Please use a nightly build.


## Building this project

1. Make sure you have all the requirements completed / installed

## Initial Benchmarks

Run `cargo bench`

```rust
test tests::qubits_16_cpu                ... bench:     220,692 ns/iter (+/- 47,403)
test tests::qubits_16_from_string_cpu    ... bench:     224,432 ns/iter (+/- 25,096)
test tests::qubits_16_from_string_opencl ... bench:       5,079 ns/iter (+/- 633)
test tests::qubits_16_opencl             ... bench:       4,965 ns/iter (+/- 543)
```

## Notes

Currently, the biggest bottleneck is the kron function, though it is optimized with tiling...

Can also max at 30 qubits, or the number storage will have to be changed (Will require using `f64` complex numbers instead of `f32`, which not all GPUs support)

## License

This software is licensed under the MIT licence (see `LICENSE.md`)

Copyright (c) 2018 Adam Kelly

## Citations

```
@misc{Yalamanchili2015,
abstract = {ArrayFire is a high performance software library for parallel computing with an easy-to-use API. Its array based function set makes parallel programming simple. ArrayFire's multiple backends (CUDA, OpenCL and native CPU) make it platform independent and highly portable. A few lines of code in ArrayFire can replace dozens of lines of parallel computing code, saving you valuable time and lowering development costs.},
address = {Atlanta},
author = {Yalamanchili, Pavan and Arshad, Umar and Mohammed, Zakiuddin and Garigipati, Pradeep and Entschev, Peter and Kloppenborg, Brian and Malcolm, James and Melonakos, John},
publisher = {AccelerEyes},
title = {{ArrayFire - A high performance software library for parallel computing with an easy-to-use API}},
url = {https://github.com/arrayfire/arrayfire},
year = {2015}
}
```

