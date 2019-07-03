#include <math.h>

#include "tangle.h"

void X(TangleState state, int target) {
  TangleGate x;
  x.A = 0;
  x.B = 1;
  x.C = 1;
  x.D = 0;

  apply_antidiagonal_gate(state, target, x);
}

void CX(TangleState state, int control, int target) {
  TangleGate x;
  x.A = 0;
  x.B = 1;
  x.C = 1;
  x.D = 0;

  apply_controlled_antidiagonal_gate(state, control, target, x);
}

void H(TangleState state, int target) {
  TangleGate u;
  u.A = 0.70710678118654752440;
  u.B = 0.70710678118654752440;
  u.C = 0.70710678118654752440;
  u.D = -0.70710678118654752440;

  apply_gate(state, target, u);
}

void Z(TangleState state, int target) {
  TangleGate u;
  u.A = 1;
  u.B = 0;
  u.C = 0;
  u.D = -1;

  apply_diagonal_gate(state, target, u);
}

void CZ(TangleState state, int control, int target) {
  TangleGate u;
  u.A = 1;
  u.B = 0;
  u.C = 0;
  u.D = -1;

  apply_controlled_diagonal_gate(state, control, target, u);
}