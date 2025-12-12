Main: main.o mario.o ../target/debug/libemu_core.a
	clang main.o mario.o ../target/debug/libemu_core.a -lpthread -ldl -lm -o Main

main.o: main.c
	clang -c main.c

mario.o:
	cd ../nesc; cargo run
	mv ../nesc/mario.o .

../target/debug/libemu_core.a:
	cd ../emu-core; cargo build

.PHONY: mario.o ../target/debug/libemu_core.a
