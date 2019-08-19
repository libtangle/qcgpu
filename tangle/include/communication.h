#ifndef __TANGLE_COMMUNICATION_H
#define __TANGLE_COMMUNICATION_H

#include "tangle.h"

void send_top(TangleState state, int node);
void receive_top(TangleState state, int node);
void send_bottom(TangleState state, int node);
void receive_bottom(TangleState state, int node);

#endif