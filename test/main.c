#include <stdio.h>

#include "interface.h"

void nes_game(State *state);

State *new_state_from_file_name(const char *name);

int main() {
	puts("Starting");
	State *state = new_state_from_file_name("../non-free/SMB1.nes");
	nes_game(state);
}
