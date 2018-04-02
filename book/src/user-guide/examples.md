# Examples

Here is a complete example of using QCGPU to simulate the bell state.

```rust
extern crate qcgpu;

use qcgpu::State;

fn main() {
    // Create a new quantum register with 2 qubits on OpenCL device #0
    let mut register = State::new(2, 0);

    // Apply a hadamard gate to the first qubit
    register.h(0);

    // Apply a CNOT, with the control = 0, and target = 1
    register.cx(0, 1);

    // Measure the register 1000 times and print the output
    println!("Measured: {:?}", register.measure_many(1000));
}
```

The output should be something like 

```shell
Measured: {"00": 516, "11": 484}
```

For more, non trivial examples, see the [Algorithms](../algorithms/algorithms.html)
