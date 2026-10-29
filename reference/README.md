# Reference Binaries

This directory contains documentation about the reference binaries used to
verify that the AGC source code assembles correctly.

## Source

The reference binaries come from the Virtual AGC project:
<https://github.com/virtualagc/virtualagc>

Specifically, the `.binsource` files in:

- `virtualagc/Comanche055/Comanche055.binsource` - Command Module (Colossus 2A)
- `virtualagc/Luminary099/Luminary099.binsource` - Lunar Module (Luminary 1A)

These binsource files are octal dumps of the original core rope memory,
transcribed from MIT documentation and cross-verified against multiple sources.

## Binsource Format

Each binsource file contains:

- Bank headers (e.g., `BANK=02`)
- Lines of 8 octal words (e.g., `50026 27064 30027 30026 50025 27066 30025 24553`)
- Bugger words (checksums) at the end of each bank
- Comment lines starting with `;`

The `tools/binsource-to-bin` converter parses this format and produces a raw
binary file (big-endian 16-bit words) that can be compared against yaYUL output.

## Verification

The CI pipeline (`agc-build.yml`) assembles the AGC source with yaYUL and
compares the output byte-for-byte against these reference binaries. A match
confirms that the transcribed source code is faithful to the original flight
software.
