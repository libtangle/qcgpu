# Quantum Operations

There are a number of operations you can preform on quantum registers with QCGPU.

## Measurement
You can measure the register in two ways. You can either do a single measurement and return an integer with the measured value or you can measure multiple times and return a `HashMap<String, i32>`, with the key being the bitstring, and the value being the number of times it was measured.

The measurements don't collapse the state.

They are used as follows:

```rust
use qcgpu::State;

let mut state = State::new(5, 0);
state.measure(); // Returns an integer
state.measure_many(1000); // Measures 1000 times, returns a HashMap<String, i32>
```

There is also a convenience method to measure the first \\(n\\) qubits in the register. Again, the state is not collapsed

```rust
use qcgpu::State;

let mut state = State::new(5, 0);
state.measure_first(3, 1000); // Measures the first 3 qubits 1000 times, returns a HashMap<String, i32>
```

## Probability
QCGPU provides another method for getting the probability of each outcome.

The probability is calculated for a state \\(\lvert \psi \rangle = \sum_{j = 0}^{2^n - 1} \alpha_j \lvert j \rangle\\), 

\\[P(j) = |\alpha_j|^2\\]

The method `get_probabilities` returns a `Vec<f32>` with each of the values corresponding to \\(|\alpha_j|^2\\) for each index \\(j\\).

```rust
use qcgpu::State;

let mut state = State::new(1, 0);
state.h(0);
state.get_probabilities(); // [0.5, 0.5]
```
