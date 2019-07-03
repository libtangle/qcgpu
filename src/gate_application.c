#include "mpi.h"
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#include "communication.h"
#include "tangle.h"

llint is_bit_clear(llint n, llint pos) { return n & (1 << pos); }

void apply_gate(TangleState state, int target, TangleGate u) {
  if (target < state.m) {
    // Communication isn't required
    llint stride = 1LL << (target + 1LL);
    for (llint i = 0; i < state.node_amps; i += stride) {
      for (llint j = i; j < i + (1LL << target); j++) {
        llint zero = j;
        llint one = j + (1LL << target);

        cfloat zero_amp = state.amps[zero];
        cfloat one_amp = state.amps[one];

        state.amps[zero] = (u.A * zero_amp) + (u.B * one_amp);
        state.amps[one] = (u.C * zero_amp) + (u.D * one_amp);
      }
    }
  } else {
    // Communication is required
    llint stride = 1LL << (target - state.m);
    llint global_state = state.rank * state.node_amps;

    llint node_i, node_j;
    if (is_bit_clear(global_state, target) == 0) {
      node_i = state.rank;
      node_j = node_i + stride;
    } else {
      node_j = state.rank;
      node_i = node_j - stride;
    }

    if (node_i == state.rank) {
      // s1 --> dt
      // d2 --> st
      send_top(state, node_j);

      for (llint i = 0; i < state.temp_amps; i++) {
        llint zero = i + state.temp_amps;
        llint one = i;

        cfloat zero_amp = (&state.amps[0])[zero];
        cfloat one_amp = (&state.amps[state.node_amps])[one];

        (&state.amps[0])[zero] = (u.A * zero_amp) + (u.B * one_amp);
        (&state.amps[state.node_amps])[one] =
            (u.C * zero_amp) + (u.D * one_amp);
      }

      // st --> d2
      // dt --> s1
      recieve_top(state, node_j);
    } else {
      // s2 --> dt
      // d1 --> st
      send_bottom(state, node_i);

      for (llint i = 0; i < state.temp_amps; i++) {
        llint zero = i;
        llint one = i;

        cfloat zero_amp = (&state.amps[state.node_amps])[zero];
        cfloat one_amp = (&state.amps[0])[one];

        (&state.amps[state.node_amps])[zero] =
            (u.A * zero_amp) + (u.B * one_amp);
        (&state.amps[0])[one] = (u.C * zero_amp) + (u.D * one_amp);
      }

      // st --> d1
      // dt --> s2
      recieve_bottom(state, node_i);
    }
  }
  MPI_Barrier(MPI_COMM_WORLD);
}

void apply_diagonal_gate(TangleState state, int target, TangleGate u) {
  // Communication can always be avoided
  for (llint i = 0; i < state.node_amps; i++) {
    if (is_bit_clear(i, target) == 0) {
      state.amps[i] = u.A * state.amps[i];
    } else {
      state.amps[i] = u.D * state.amps[i];
    }
  }

  MPI_Barrier(MPI_COMM_WORLD);
}

void apply_antidiagonal_gate(TangleState state, int target, TangleGate u) {
  if (target < state.m) {
    // Communication isn't required
    llint stride = 1LL << (target + 1LL);
    for (llint i = 0; i < state.node_amps; i += stride) {
      for (llint j = i; j < i + (1LL << target); j++) {
        llint zero = j;
        llint one = j + (1LL << target);

        cfloat zero_amp = state.amps[zero];
        cfloat one_amp = state.amps[one];

        state.amps[zero] = u.B * one_amp;
        state.amps[one] = u.C * zero_amp;
      }
    }
  } else {
    // Communication is required
    llint stride = 1LL << (target - state.m);
    llint global_state = state.rank * state.node_amps;

    llint node_i, node_j;
    if (is_bit_clear(global_state, target) == 0) {
      node_i = state.rank;
      node_j = node_i + stride;
    } else {
      node_j = state.rank;
      node_i = node_j - stride;
    }

    if (node_i == state.rank) {
      // s1 --> dt
      // d2 --> st
      send_top(state, node_j);

      for (llint i = 0; i < state.temp_amps; i++) {
        llint zero = i + state.temp_amps;
        llint one = i;

        cfloat zero_amp = (&state.amps[0])[zero];
        cfloat one_amp = (&state.amps[state.node_amps])[one];

        (&state.amps[0])[zero] = u.B * one_amp;
        (&state.amps[state.node_amps])[one] = u.C * zero_amp;
      }

      // st --> d2
      // dt --> s1
      recieve_top(state, node_j);
    } else {
      // s2 --> dt
      // d1 --> st
      send_bottom(state, node_i);

      for (llint i = 0; i < state.temp_amps; i++) {
        llint zero = i;
        llint one = i;

        cfloat zero_amp = (&state.amps[state.node_amps])[zero];
        cfloat one_amp = (&state.amps[0])[one];

        (&state.amps[state.node_amps])[zero] = u.B * one_amp;
        (&state.amps[0])[one] = u.C * zero_amp;
      }

      // st --> d1
      // dt --> s2
      recieve_bottom(state, node_i);
    }
  }
  MPI_Barrier(MPI_COMM_WORLD);
}

void apply_controlled_gate(TangleState state, int control, int target,
                           TangleGate u) {
  if (target < state.m) {
    // Communication isn't required
    llint stride = 1LL << (target + 1LL);
    // TODO: Better stride for controlled gates
    for (llint i = 0; i < state.node_amps; i += stride) {
      for (llint j = i; j < i + (1LL << target); j++) {
        if (is_bit_clear(j, control) != 0) {
          llint zero = j;
          llint one = j + (1LL << target);

          cfloat zero_amp = state.amps[zero];
          cfloat one_amp = state.amps[one];

          state.amps[zero] = (u.A * zero_amp) + (u.B * one_amp);
          state.amps[one] = (u.C * zero_amp) + (u.D * one_amp);
        }
      }
    }
  } else {
    // Communication is required
    llint stride = 1LL << (target - state.m);
    llint global_state = state.rank * state.node_amps;

    llint node_i, node_j;

    if (is_bit_clear(global_state, target) == 0) {
      node_i = state.rank;
      node_j = node_i + stride;
    } else {
      node_j = state.rank;
      node_i = node_j - stride;
    }

    if (node_i == state.rank) {
      // s1 --> dt
      // d2 --> st
      send_top(state, node_j);

      for (llint i = 0; i < state.temp_amps; i++) {
        llint zero = i + state.temp_amps;
        llint one = i;

        if (is_bit_clear(zero, control) != 0) {
          cfloat zero_amp = (&state.amps[0])[zero];
          cfloat one_amp = (&state.amps[state.node_amps])[one];

          (&state.amps[0])[zero] = (u.A * zero_amp) + (u.B * one_amp);
          (&state.amps[state.node_amps])[one] =
              (u.C * zero_amp) + (u.D * one_amp);
        }
      }

      // st --> d2
      // dt --> s1
      recieve_top(state, node_j);
    } else {
      // s2 --> dt
      // d1 --> st
      send_bottom(state, node_i);

      for (llint i = 0; i < state.temp_amps; i++) {
        llint zero = i;
        llint one = i;

        if (is_bit_clear(zero, control) != 0) {

          cfloat zero_amp = (&state.amps[state.node_amps])[zero];
          cfloat one_amp = (&state.amps[0])[one];

          (&state.amps[state.node_amps])[zero] =
              (u.A * zero_amp) + (u.B * one_amp);
          (&state.amps[0])[one] = (u.C * zero_amp) + (u.D * one_amp);
        }
      }
      // st --> d1
      // dt --> s2
      recieve_bottom(state, node_i);
    }
  }
  MPI_Barrier(MPI_COMM_WORLD);
}

// There is some faster way of doing this
// TODO: implement
void apply_controlled_diagonal_gate(TangleState state, int control, int target,
                                    TangleGate u) {
  apply_controlled_gate(state, control, target, u);
}

// There is some faster way of doing this
// TODO: implement
void apply_controlled_antidiagonal_gate(TangleState state, int control,
                                        int target, TangleGate u) {
  apply_controlled_gate(state, control, target, u);
}