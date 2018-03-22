/* Applies a single qubit gate to the register.
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
