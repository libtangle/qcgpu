#ifndef __COMMUNICATION_H
#define __COMMUNICATION_H

#include "tangle.h"

void send_top(TangleState state, int node);
void recieve_top(TangleState state, int node);
void send_bottom(TangleState state, int node);
void recieve_bottom(TangleState state, int node);

#endif