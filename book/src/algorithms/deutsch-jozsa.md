# Deutsch-Jozsa Algorithm

This algorithm was the first to show that quantum computers could have a speedup over classical computers.

Consider a function \\(f(x)\\) which takes an input of an \\(n\\)-bit string \\(x\\) and returns 0 or 1.

Suppose that \\(f(x)\\) is either a **constant** function that has the same value \\(c \in \{0, 1\},  \forall x\\), or a **balanced** function, where the value is 0 for half of the inputs, and 1 for the other half.

The Deutsch-Jozsa problem is to find whether \\(f\\) is *constant* or *balanced*, in as few function evaluations as possible.

Using classical computing, in the worst case, this requires \\(2^{n-1}+1\\) function evaluations.
Using quantum computing, this can be done with just one function evaluation.

The function \\(f\\), to be used in a quantum computer, must be specified by an oracle circuit \\(U_{f}\\) such that \\(U_{f} \lvert x \rangle = (-1)^{f(x)}\lvert x \rangle\\).

## The Algorithm

1. Initialize \\(n\\) qubits in the state \\(\lvert 0, \dots, 0\rangle\\).
2. Apply the Hadamard gate \\(H\\) to each qubit.
3. Apply the oracle circuit \\(U_f\\).
4. Apply the Hadamard gate \\(H\\) to each qubit.
5. Measure each qubit. Let \\(y = (y_{1}, \dots, y_{n})\\) be the list of measurement outcomes.

From this procedure, we find that \\(f\\) is constant if \\(y\\) is the all zero string.

```rust
extern crate qcgpu;

use qcgpu::State;
use qcgpu::gates::h;

fn main() {
    // 3 qubits, f(x) = x_0 NOT x_1 x_2
    // Balanced
    let mut balanced_state = State::new(3, 1);

    balanced_state.apply_all(h());

    // Oracle U_f
    balanced_state.h(2);
    balanced_state.z(0);
    balanced_state.cx(1, 2);
    balanced_state.h(2);

    balanced_state.apply_all(h());

    println!(
        "{}",
        if balanced_state.measure() == 0 {
            "constant"
        } else {
            "balanced"
        }
    );

    // 3 qubits, f(x) = 0
    // Constant
    let mut constant_state = State::new(3, 1);

    constant_state.apply_all(h());

    // Oracle is equivilant to the identity gate, thus has no effect on the
    // state.

    constant_state.apply_all(h());

    println!(
        "{}",
        if constant_state.measure() == 0 {
            "constant"
        } else {
            "balanced"
        }
    );
}
```
