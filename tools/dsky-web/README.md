# DSKY Web Emulator

A browser-based emulator of the Apollo DSKY (Display/Keyboard) unit that
connects to the yaAGC simulator via a WebSocket bridge.

## Architecture

```
Browser (index.html)  <--WebSocket-->  dsky-bridge  <--TCP-->  yaAGC
   SVG 7-segment                      (Rust binary)          (AGC sim)
   display + keyboard                 port 8765              port 19697
```

## Quick Start

1. Start yaAGC with an assembled binary:
   ```
   yaAGC --port=19697 Comanche055/MAIN.agc.bin
   ```

2. Build and run the bridge:
   ```
   cd tools/dsky-web
   cargo run
   ```

3. Open `tools/dsky-web/public/index.html` in a browser.

## Keyboard Shortcuts

| Key | DSKY Function |
|-----|--------------|
| `v` | VERB |
| `n` | NOUN |
| `0`-`9` | Digits |
| `Enter` | ENTR |
| `Backspace` | CLR |
| `+` / `-` | Plus / Minus |
| `r` | RSET (Error Reset) |

## Display

The emulator renders all DSKY elements:
- **PROG**: 2-digit program number
- **VERB/NOUN**: 2-digit verb and noun
- **R1, R2, R3**: 5-digit signed registers with +/- indicators
- **14 indicator lights**: COMP ACTY, UPLINK ACTY, PROG ALARM, etc.

Seven-segment digits are rendered as SVG paths matching the physical
DSKY's electroluminescent display segments.

## Protocol

The bridge translates yaAGC's 4-byte binary channel protocol into JSON:

```json
{"type": "channel", "channel": 8, "value": 12345}
{"type": "key", "code": 17}
```
