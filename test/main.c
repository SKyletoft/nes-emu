#include <stdio.h>

#include "interface.h"

void bb_8000(State *state);

State *new_state_from_file_name(const char *name);

int main() {
	State *state = new_state_from_file_name("../non-free/SMB1.nes");
	bb_8000(state);
}
