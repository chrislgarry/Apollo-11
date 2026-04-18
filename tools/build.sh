#!/usr/bin/env bash
# Assemble an AGC program directory with yaYUL.
#
# Usage: ./tools/build.sh <program_dir> [yaYUL_path]
#   program_dir: Comanche055 or Luminary099
#   yaYUL_path:  path to yaYUL binary (default: yaYUL on PATH)
#
# Produces:
#   <program_dir>/MAIN.agc.bin  - the assembled rope image
#   <program_dir>/MAIN.agc.lst  - assembly listing
#
# Exit codes:
#   0 - assembly succeeded
#   1 - assembly failed or missing arguments

set -euo pipefail

PROGRAM_DIR="${1:?Usage: $0 <program_dir> [yaYUL_path]}"
YAYUL="${2:-yaYUL}"

if [ ! -d "$PROGRAM_DIR" ]; then
    echo "Error: directory '$PROGRAM_DIR' not found" >&2
    exit 1
fi

if [ ! -f "$PROGRAM_DIR/MAIN.agc" ]; then
    echo "Error: '$PROGRAM_DIR/MAIN.agc' not found" >&2
    exit 1
fi

if ! command -v "$YAYUL" &>/dev/null; then
    echo "Error: yaYUL not found at '$YAYUL'. Build it from virtualagc or add to PATH." >&2
    exit 1
fi

echo "Assembling $PROGRAM_DIR with $YAYUL..."
cd "$PROGRAM_DIR"

"$YAYUL" MAIN.agc

if [ -f "MAIN.agc.bin" ]; then
    echo "Assembly succeeded: $(wc -c < MAIN.agc.bin) bytes"
    exit 0
else
    echo "Assembly failed: no .bin file produced" >&2
    exit 1
fi
