# Quantum Registers

All of the simulation is done through quantum registers. QCGPU provides a struct as a register, but that contains fields to do with the OpenCL buffers and related items, so the creation of registers should be done through the provided methods.

The library is optimized for complex superpositions, so the registers are all dense. This means that the number of qubits you can initialize is directly related to the capacity / available memory of the device.

The register struct is called `State` and is available through `qcgpu::State`.

To create a register, the easiest way to do it is with the `State::new` method.
It takes two parameters, the number of qubits and the device to use. The device is given as a usize, and corresponds to the OpenCL device which the register will be on.

The following example creates a register with 5 qubits on the 1st device.

```rust
# extern crate qcgpu;

use qcgpu::State;

# fn main() {
let mut register = State::new(5, 0);
# }
```

Notice that the register is mutable. This allows the register to change. Also, the device is 0 indexed.

The implementation is equivilent to the description of a state vector \\( \lvert \psi \rangle \\) with

\\[ \lvert \psi \rangle = \sum_{j = 0}^{2^n - 1} \alpha_j \lvert j \rangle \\]

where \\(n\\) is the number of qubits, \\(\alpha_j\\) is the amplitude and the state is \\(j\\) runs over all \\(2^n\\) basis states.

There is one other way to initialize a state. Given a bitstring, a register can be initialized in that state using the `State::from_bit_string` method. For example, to initialize a register with the value `0100` the following is used:

```rust
# extern crate qcgpu;

use qcgpu::State;

# fn main() {
let mut register = State::from_bit_string("|0100>", 0);
# }
```

The second argument is the same as before. The register that is outputed from this method is equivilent to the state

\\[ \lvert 0100 \rangle\\]


