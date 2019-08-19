#include <math.h>
#include "tangle.h"

// Predefined Gates
TangleGate _X = {.A = 0, .B = 1, .C = 1, .D = 0};
TangleGate _Z = {.A = 1, .B = 0, .C = 0, .D = -1};
TangleGate _H = {.A = M_SQRT1_2, .B = M_SQRT1_2, .C = M_SQRT1_2, .D = -M_SQRT1_2};

// Shorthand Gate Application Functions
void X(TangleState state, int target)
{
    apply_antidiagonal_gate(state, target, _X);
}

void Z(TangleState state, int target)
{
    apply_diagonal_gate(state, target, _Z);
}

void H(TangleState state, int target)
{
    apply_gate(state, target, _H);
}