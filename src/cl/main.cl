/**
 * Applies a single qubit gate to the register.
 * The gate matrix must be given in the form:
 *
 *  A B
 *  C D
 */
__kernel void apply_gate(
  __global float* const amplitudes,
  __global float* amps,
  uint target,
  float A,
  float B,
  float C,
  float D
) {
  uint const state = get_global_id(0);
  float const amp = amplitudes[state];

  uint const zero_state = state & (~(1 << target));
  uint const one_state = state | (1 << target);

  uint const bit_val = (((1 << target) & state) > 0)? 1 : 0;

  if (bit_val == 0) {
    // Bitval = 0
    amps[state] = (A * amp) + (B * amplitudes[one_state]);
  } else {
    amps[state] = (D * amp) + (C * amplitudes[zero_state]);
  }
}


/**
 * Applies a controlled single qubit gate to the register.
 */
__kernel void apply_controlled_gate(
  __global float* const amplitudes,
  __global float* amps,
  uint control,
  uint target,
  float A,
  float B,
  float C,
  float D
) {
  uint const state = get_global_id(0);
  float const amp = amplitudes[state];

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
        amps[state] = (A * amp) + (B * amplitudes[one_state]);
    } else {
        amps[state] = (D * amp) + (C * amplitudes[zero_state]);
    }
  }
}
