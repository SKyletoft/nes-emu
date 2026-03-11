An NES emulator that recompiles the entire game to native code at
compile time. Only supports NROM games at the moment.

Also has a 3DS build (`cargo 3ds run -r -p frontend`).

## Demo (YouTube link)

[![Youtube video](https://i3.ytimg.com/vi/RqFTMtwv6ao/maxresdefault.jpg)](https://www.youtube.com/watch?v=RqFTMtwv6ao)

# Usage

The emulator is built around static recompilation. That means ROMs
have to be provided at compile time. The path to the ROM is to be
passed to the `compile_nes_to_rust!`-macro in the `game` crate.

Expect compile times of 3-10 minutes as dead code elimination is still very bad.

# Default controls (Desktop)

D-pad: Arrow keys

A: Z

B: X

Start: Enter

# Default controls (3DS)

D-pad: D-pad (Circlepad is unbound)

A: A

B: B/X

Start: Start

Widescreen: L/R
