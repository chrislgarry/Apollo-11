#!/usr/bin/env bash
# Compare an assembled AGC binary against a known-good reference binsource.
#
# Usage: ./tools/verify-binary.sh <program_dir> <reference_binsource>
#   program_dir:         Comanche055 or Luminary099
#   reference_binsource: path to the reference .binsource file from virtualagc
#
# The binsource file is converted to raw binary using tools/binsource-to-bin,
# then compared byte-for-byte against the yaYUL output.
#
# Exit codes:
#   0 - binaries match
#   1 - binaries differ or error

set -euo pipefail

PROGRAM_DIR="${1:?Usage: $0 <program_dir> <reference_binsource>}"
BINSOURCE="${2:?Usage: $0 <program_dir> <reference_binsource>}"

ASSEMBLED="$PROGRAM_DIR/MAIN.agc.bin"
REFERENCE_BIN="/tmp/reference_${PROGRAM_DIR##*/}.bin"
CONVERTER="$(dirname "$0")/binsource-to-bin"

if [ ! -f "$ASSEMBLED" ]; then
    echo "Error: assembled binary '$ASSEMBLED' not found. Run build.sh first." >&2
    exit 1
fi

if [ ! -f "$BINSOURCE" ]; then
    echo "Error: reference binsource '$BINSOURCE' not found." >&2
    exit 1
fi

# Build the converter if needed
if [ ! -x "$CONVERTER" ]; then
    CONVERTER_SRC="$(dirname "$0")/binsource-to-bin.rs"
    if [ -f "$CONVERTER_SRC" ]; then
        echo "Building binsource converter..."
        rustc "$CONVERTER_SRC" -o "$CONVERTER"
    else
        echo "Error: binsource converter not found at '$CONVERTER' or '$CONVERTER_SRC'" >&2
        exit 1
    fi
fi

echo "Converting reference binsource to binary..."
"$CONVERTER" "$BINSOURCE" "$REFERENCE_BIN"

echo "Comparing binaries..."
if cmp -s "$ASSEMBLED" "$REFERENCE_BIN"; then
    echo "PASS: Assembled binary matches reference"
    echo "  Assembled:  $(sha256sum "$ASSEMBLED" | cut -d' ' -f1)"
    echo "  Reference:  $(sha256sum "$REFERENCE_BIN" | cut -d' ' -f1)"
    rm -f "$REFERENCE_BIN"
    exit 0
else
    echo "FAIL: Assembled binary differs from reference" >&2
    echo "  Assembled size: $(wc -c < "$ASSEMBLED") bytes"
    echo "  Reference size: $(wc -c < "$REFERENCE_BIN") bytes"

    # Show first few differing words
    echo "  First differences:"
    cmp -l "$ASSEMBLED" "$REFERENCE_BIN" 2>/dev/null | head -10 | while read -r offset a_byte r_byte; do
        printf "    Offset %s: assembled=0%03o reference=0%03o\n" "$offset" "$a_byte" "$r_byte"
    done

    rm -f "$REFERENCE_BIN"
    exit 1
fi
