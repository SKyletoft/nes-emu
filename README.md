An NES emulator that recompiles the entire game to native code at
compile time. Only supports NROM games at the moment.

Also has a 3DS build (`cargo 3ds run -r -p frontend`).

## Demo

<iframe width="560" height="315" src="https://www.youtube.com/embed/RqFTMtwv6ao?si=219cC0v_6--k4CSl" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

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
