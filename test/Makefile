run: Main
	./Main

Main: main.o mario.o ../target/debug/libemu_core.a
	clang main.o mario.o ../target/debug/libemu_core.a -o Main

main.o: main.c
	clang -c main.c -I ../emu-core/inc

mario.o:
	cd ../nesc; cargo run
	mv ../nesc/mario.o .

../target/debug/libemu_core.a:
	cd ../emu-core; cargo build

.PHONY: mario.o ../target/debug/libemu_core.a run
