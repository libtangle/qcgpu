# Super Dense Coding

if Alice and Bob share a pair of entangled qubits, then Alice can encode two classical bits into her one entangled qubit,
send it to Bob, and Bob can decode it with the help of his entangled qubit.

```rust
extern crate qcgpu;

use qcgpu::State;

fn superdense(input: &str) -> i32 {
    let mut state = State::new(2, 0);
    let input_str = String::from(input);

    // Prepare the bell state
    state.h(0);
    state.cx(0, 1);

    // Alice prepares her qubit
    let alice = 1;
    if input_str.get(0..1) == Some("1") {
        state.z(alice);
    }
    if input_str.get(1..2) == Some("1") {
        state.x(alice);
    }

    println!("\nState after Alice prepares her qubit: \n{}", state);

    // Alice sends her qubit to Bob
    let bob = 0;
    state.cx(alice, bob);
    state.h(alice);

    println!(
        "\nState after Bob receives Alice's qubit and 'decodes' it: \n{}",
        state
    );

    state.measure()
}

fn main() {
    use std::io;

    println!("Two bit string to send:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            let result = superdense(input.as_str());
            println!("\nDecoded string is: {}", result);
        }
        Err(error) => println!("error: {}", error),
    }
}
```
