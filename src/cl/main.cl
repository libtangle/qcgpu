typedef float2 complex_f;

complex_f add(complex_f a, complex_f b) {
    return (complex_f)(a.x + b.x, a.y + b.y);
}

complex_f mul(complex_f a, complex_f b) {
    return (complex_f)(
      (a.x * b.x) - (a.y * b.y),
      (a.y * b.x) + (a.x * b.y)
    );
}

/**
 * Applies a single qubit gate to the register.
 * The gate matrix must be given in the form:
 *
 *  A B
 *  C D
 */
__kernel void apply_gate(
  __global complex_f* const amplitudes,
  __global complex_f* amps,
  uint target,
  complex_f A,
  complex_f B,
  complex_f C,
  complex_f D
) {
  uint const state = get_global_id(0);
  complex_f const amp = amplitudes[state];

  uint const zero_state = state & (~(1 << target));
  uint const one_state = state | (1 << target);

  uint const bit_val = (((1 << target) & state) > 0)? 1 : 0;

  if (bit_val == 0) {
    // Bitval = 0

    amps[state] = add(mul(A, amp), mul(B, amplitudes[one_state]));
  } else {
    amps[state] = add(mul(D, amp), mul(C, amplitudes[zero_state]));
  }
}


/**
 * Applies a controlled single qubit gate to the register.
 */
__kernel void apply_controlled_gate(
  __global complex_f* const amplitudes,
  __global complex_f* amps,
  uint control,
  uint target,
  complex_f A,
  complex_f B,
  complex_f C,
  complex_f D
) {
  uint const state = get_global_id(0);
  complex_f const amp = amplitudes[state];

  uint const zero_state = state & (~(1 << target));
  uint const one_state = state | (1 << target);

  uint const bit_val = (((1 << target) & state) > 0)? 1 : 0;
  uint const control_val = (((1 << control) & state) > 0)? 1 : 0;

  if (control_val == 0) {
    // Control is 0, don't apply gate
    amps[state] = amp;
  } else {
    // control is 1, apply gate.
    if (bit_val == 0) {
        // Bitval = 0
        amps[state] = add(mul(A, amp), mul(B, amplitudes[one_state]));
    } else {
        amps[state] = add(mul(D, amp), mul(C, amplitudes[zero_state]));
    }
  }
}
