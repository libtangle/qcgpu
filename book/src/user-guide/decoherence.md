# Decoherence

QCGPU provides a way to easily simulate the effects of decoherence on the quantum computer. The effects are simulated by a random gate, corresponding to a random rotation around the \\(z\\) axis of the bloch sphere.

The angle of the rotation is a normal distrobuted value, with the varience as the strength factor `d`.

To avoid performance costs when not being used, decoherence can be enabled via a feature.

Change the dependency in `cargo.toml` to

```toml
[dependencies]
qcgpu = { version = "0.1", features = ["decoherence"] }
```

Now you can add decoherence to your simulator.

To set the amount of decoherence, the `set_decoherence` method is used. Its arguments are the strength of the decoherence.
This will affect all following gate applications.

```rust
use qcgpu::State;

let mut register = State::new(5, 0);
register.set_decoherence(0.4);
```

You can also manually decohere the register based on the previously set strength value. This method is automatically called by all gate applications.

```rust
register.decohere();
```
