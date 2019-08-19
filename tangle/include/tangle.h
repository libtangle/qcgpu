// The outward facing API for Tangle

#ifndef __TANGLE_H
#define __TANGLE_H

#include <complex.h>

// Common Types
typedef float _Complex cfloat;
typedef long long int llint;

#define cfloat(r, i) ((float)(r) + ((float)(i)) * I)

// Tangle Types
typedef struct
{
    int rank;
    int nodes;
} TangleEnvironment;

typedef struct
{
    int num_qubits;
    cfloat *amps;

    // Distributed Information
    int rank;
    int nodes;
    int k;
    int m;

    llint node_amps;
    llint temp_amps;
} TangleState;

typedef struct
{
    cfloat A;
    cfloat B;
    cfloat C;
    cfloat D;
} TangleGate;

// Environment/Distributed Functions
TangleEnvironment initialize_environment();
void destroy_environment(TangleEnvironment env);

// Quantum State Functions
TangleState create_state(int num_qubits, TangleEnvironment env);
void print_state(TangleState state);

// Quantum Gates
void apply_gate(TangleState state, int target, TangleGate u);
void apply_diagonal_gate(TangleState state, int target, TangleGate u);
void apply_antidiagonal_gate(TangleState state, int target, TangleGate u);
void apply_controlled_gate(TangleState state, int control, int target, TangleGate u);
void apply_controlled_diagonal_gate(TangleState state, int control, int target, TangleGate u);
void apply_controlled_antidiagonal_gate(TangleState state, int control, int target, TangleGate u);

TangleGate _X;
TangleGate _Z;
TangleGate _H;

void X(TangleState state, int target);
void H(TangleState state, int target);
void Z(TangleState state, int target);
void CZ(TangleState state, int control, int target);
void CX(TangleState state, int control, int target);

// Measurement
llint measure(TangleState state);

#endif