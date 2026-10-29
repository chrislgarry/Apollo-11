# Apollo 11 AGC Modernization -- Phase C: Testing & CI Infrastructure

## Context

The Apollo 11 AGC repository (chrislgarry/Apollo-11) contains 130,186 lines of AGC assembly across 175 `.agc` files in two modules -- Comanche055 (Command Module / Colossus 2A) and Luminary099 (Lunar Module / Luminary 1A). The repo is a preservation project with **zero build, test, or simulation infrastructure** -- only markdown linting exists in CI. Before any algorithmic optimization (Category A) or modern reimplementation (Category B) can be safely attempted, we need a verification foundation.

This plan builds that foundation in 4 phases, progressing from assembly verification through arithmetic unit testing, integration testing via simulation, and restart/recovery fuzzing.

---

## Phase 1: CI Pipeline -- Assembly & Binary Verification

**Goal**: Assemble both modules with yaYUL on every push/PR and compare output against known-good reference binaries.

### Files to Create

| File | Purpose |
|------|---------|
| `.github/workflows/agc-build.yml` | CI workflow: build yaYUL, assemble both programs, verify binaries |
| `tools/Dockerfile` | Reproducible build environment with yaYUL + yaAGC |
| `tools/build.sh` | Assemble a program directory with yaYUL |
| `tools/verify-binary.sh` | Compare assembled binary against reference binsource |
| `tools/binsource-to-bin.rs` | Convert Virtual AGC binsource (octal text) to raw binary |
| `reference/README.md` | Document provenance of reference binaries |

### Key Design Decisions

- **yaYUL built from source** (Virtual AGC repo, pinned commit hash) -- no pre-built binaries available, builds in <30s
- **Reference binaries** from `virtualagc/virtualagc` repo binsource files (octal dumps of original core rope)
- **Docker image** for hermetic builds, published to GHCR after first build
- **Matrix strategy**: Parallel jobs for Comanche055 and Luminary099

### CI Workflow Structure

```yaml
on: [push, pull_request]
jobs:
  assemble:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        program: [Comanche055, Luminary099]
    steps:
      - checkout
      - clone virtualagc (shallow, pinned commit)
      - build yaYUL from source
      - run tools/build.sh ${{ matrix.program }}
      - run tools/verify-binary.sh ${{ matrix.program }}
      - upload .bin and .lst as artifacts
```

### Validation

- Intentionally introduce a typo in an `.agc` file -> assembly should fail
- Unmodified sources -> bit-identical output to reference

### User Implementation Opportunity

**`tools/binsource-to-bin.rs`** -- The binsource format has bank headers, checksums, and octal word lines. The conversion logic involves parsing this format and emitting raw 16-bit big-endian words in the correct bank order. Well-bounded (~80 lines of Rust) where format understanding matters. Can live in the `agc-math` workspace as a `[[bin]]` target or be a standalone file.

---

## Phase 2: Interpreter Arithmetic Test Harness

**Goal**: Unit test every interpreter pseudo-instruction against 15-bit 1's complement fixed-point semantics.

### Technology: Rust + `proptest`

- **Rust newtypes** (`AgcWord`, `DpWord`, `TpWord`) enforce 15-bit width at the type level -- no accidental arbitrary-precision masking bugs
- **`proptest` crate** for property-based testing (commutativity, inverse, identity properties)
- **Exact integer control**: `i16`/`i32` arithmetic matches AGC bit widths precisely; overflow is explicit, not hidden
- **Performance**: Fast enough for Phase 4 fuzzing later -- the arithmetic model becomes the core of the fuzzer
- **Evolution path**: This Rust crate becomes the foundation for Category B modern reimplementations

### Files to Create

```
tests/agc-math/
  Cargo.toml                    # Rust workspace member, depends on proptest
  src/
    lib.rs                      # Public API
    ones_complement.rs          # AgcWord (SP), DpWord (DP), TpWord (TP), overflow, CCS
    arithmetic.rs               # DAD, DSU, BDSU, DMP, DMPR, DDV, BDDV, DSQ
    trig.rs                     # SINE/COSINE with exact AGC Hastings polynomial coefficients
    sqrt.rs                     # SQRTSUB with normalization (Newton-Raphson)
    vector.rs                   # DOT, VXV, MXV, VXM, UNIT, ABVAL
    poly.rs                     # POLY/POWRSERS evaluation engine
    constants.rs                # POSMAX, QUARTER, NEG1/2, BIT14, from FIXED_FIXED_CONSTANT_POOL
    mpac.rs                     # MPAC state machine (accumulator + mode register)
  tests/
    ones_complement_tests.rs    # +0 vs -0, overflow, sign agreement, CCS
    dp_arithmetic_tests.rs      # DAD, DSU, BDSU, DMP, DMPR, DDV, BDDV, DSQ
    trig_tests.rs               # SINE, COSINE, ARCSIN, ARCCOS
    sqrt_tests.rs               # SQRT, normalization
    vector_ops_tests.rs         # DOT, VXV, MXV, UNIT, ABVAL
    poly_tests.rs               # POLY evaluation with actual sine/arccos coefficients
```

### Key Constants from Source (INTERPRETER.agc)

**Sine polynomial (4th order Hastings, lines 2682-2688):**

```
+.3926990796, -.6459637111, +.318758717, -.074780249, +.009694988
```

**Arccos polynomial (7th order Hastings, lines 2758-2767):**

```
+.353553385, -.0483017006 B+1, +.0200273085 B+2, -.0112931863 B+3,
+.00695311612 B+4, -.00384617957 B+5, +.001501297736 B+6, -.000284160334 B+7
```

**Sine identity**: `SIN(X)` computes `(1/2)sin(2*pi*X)` -- argument in revolutions, result half-scaled.
**Cosine identity**: `COS(X) = SIN(PI/2 - |X|)` (line 2641).

### 1's Complement Word Model (`ones_complement.rs`)

Must model:

- **15-bit word**: values -16383 to +16383, both +0 (00000) and -0 (77777 octal)
- **Double precision (DP)**: two 15-bit words, sign bits agree, 28 magnitude bits total
- **Triple precision (TP)**: three words, as used by MPAC
- **Overflow**: 16-bit accumulator, overflow when bits 14 and 15 disagree
- **CCS semantics**: Count, Compare, and Skip -- the AGC's only conditional branch

### Test Categories

| Test File | Operations | Key Properties |
|-----------|-----------|---------------|
| `dp_arithmetic_tests.rs` | DAD, DSU, DMP, DDV, DSQ | `DAD(a,b) == DAD(b,a)`, `DDV(DMP(a,b), b) ~= a`, `DSQ(a) == DMP(a,a)` |
| `trig_tests.rs` | SINE, COSINE, ARCSIN, ARCCOS | `SIN^2+COS^2~=1`, `SIN(ASIN(x))~=x`, 256-point sweep |
| `sqrt_tests.rs` | SQRT | `SQRT(DSQ(x)) == |x|`, `SQRT(0) == 0` |
| `vector_ops_tests.rs` | DOT, VXV, MXV, UNIT, ABVAL | `DOT(a,b)==DOT(b,a)`, `VXV(a,b)==-VXV(b,a)`, `DOT(VXV(a,b),a)==0` |
| `poly_tests.rs` | POLY | Verified against actual AGC coefficient sets |

**Tolerance**: 1-2 LSBs (AGC truncates rather than rounds in most multiply ops).

### User Implementation Opportunities

1. **DDV/BDDV division algorithm** -- Lines 1792-1900+ of INTERPRETER.agc have a complex normalization loop with DVSIGN, DVNORMCT, MAXDVSW. Requires careful study of the assembly to implement the exact truncation behavior.

2. **SQRTSUB** -- Lines 2501-2580+, Newton-Raphson with normalization. The shift count returned in MPTEMP is critical for correct scaling.

### Rust Type Design

```rust
/// A 15-bit 1's complement word. Stored as i16 internally,
/// but invariants enforce the range [-16383, +16383] with
/// distinct +0 (0x0000) and -0 (0x7FFF / octal 77777).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AgcWord(i16);

/// Double-precision: two AgcWords where sign bits agree.
/// 28 magnitude bits total.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DpWord { pub major: AgcWord, pub minor: AgcWord }

/// Triple-precision: three AgcWords, as used by MPAC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TpWord { pub hi: AgcWord, pub mid: AgcWord, pub lo: AgcWord }

/// 16-bit accumulator with overflow detection.
/// Overflow = bits 14 and 15 disagree.
#[derive(Clone, Copy, Debug)]
pub struct Accumulator(i16);  // full 16-bit range
```

### CI Integration

Add to `agc-build.yml`:

```yaml
test-arithmetic:
  runs-on: ubuntu-latest
  steps:
    - checkout
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo test --manifest-path tests/agc-math/Cargo.toml -- --nocapture
```

---

## Phase 3: Integration Testing via yaAGC Simulator

**Goal**: Run assembled binaries in yaAGC, script DSKY inputs, verify outputs for key mission scenarios.

### Architecture

yaAGC communicates with peripherals over TCP sockets using a 4-byte packet protocol (channel number + value). A Rust test driver connects as a virtual DSKY. This lives in the same Cargo workspace as the arithmetic tests.

### Files to Create

```
tests/agc-integration/
  Cargo.toml                       # Workspace member, depends on agc-math + tokio
  src/
    lib.rs
    dsky_client.rs                 # DSKY socket protocol: press_key(), verb_noun(), read_display()
    agc_runner.rs                  # Start/stop yaAGC child process with a .bin file
    channel_protocol.rs            # 4-byte packet encode/decode (channel + value)
    dsky_display.rs                # Parse display state from channel data
  tests/
    fresh_start.rs                 # Boot -> PROG 00, no alarms
    verb_noun_basic.rs             # V16N36 (time), V05N09 (alarms), V35E (lamp test)
    alarm_codes.rs                 # Trigger 1202/1203, verify FAILREG + PROG ALARM light
    program_change.rs              # V37E to change major mode
    p63_p64_transition.rs          # LM landing sequence (Luminary099)
tools/
  run-integration-tests.sh         # Build binaries, start yaAGC, run cargo test
```

### DSKY Protocol

- **Channel 010 (octal)**: Keyboard input -- 5-bit key codes
- **Channel 011 (octal)**: Display output -- digit data
- **Channel 013 (octal)**: Discrete outputs -- warning lights

### Test Scenarios

| Test | Description | Expected |
|------|------------|----------|
| `fresh_start` | Boot with Comanche055.bin | PROG=00, no PROG ALARM |
| `verb_noun_basic` | V16N36E (display time) | R1/R2/R3 show advancing time |
| `verb_noun_basic` | V35E (lamp test) | All segments illuminate |
| `alarm_codes` | Exhaust job table | Alarm 1202 in FAILREG |
| `program_change` | V37E00E | Mode change to P00 |
| `p63_p64_transition` | Landing sequence (LM only) | WCHPHASE transitions -1->0->1->2 |

### User Implementation Opportunity

**`p63_p64_transition.rs`** -- Requires setting up correct initial state vectors (position, velocity, landing site, IMU alignment) before entering P63. This is where deep AGC/mission knowledge matters -- choosing realistic initial conditions that exercise the guidance transitions without crashing the simulation.

### Dependencies

- Phase 1 `.bin` files required
- Docker image must include yaAGC alongside yaYUL
- Tests marked `#[ignore]` by default, run with `cargo test -- --ignored` (slower)

---

## Phase 4: Restart/Recovery Fuzzing

**Goal**: Inject random state corruption into erasable memory and verify FRESH_START_AND_RESTART recovery.

### Architecture

Build on Phase 3's yaAGC infrastructure. Use yaAGC's `--resume` and erasable memory save/load for deterministic fuzzing.

### Files to Create

```
tests/agc-fuzzer/
  Cargo.toml                    # Workspace member, depends on agc-math, agc-integration, proptest
  src/
    lib.rs
    memory_corruptor.rs         # Selective erasable memory corruption
    restart_injector.rs         # GOJAM injection via yaAGC debug interface
    state_validator.rs          # Post-restart state validation
    coverage_tracker.rs         # Map PC addresses to source lines via .lst file
    strategies.rs               # proptest strategies for corruption patterns
  tests/
    restart_basic.rs            # Clean restart baseline
    phase_table_fuzz.rs         # Corrupt PHASE1-6, verify alarm 1107 or recovery
    erasable_fuzz.rs            # Random erasable corruption, verify no hang
```

### Corruption Targets (from FRESH_START_AND_RESTART.agc)

| Target | Addresses | Expected Recovery |
|--------|----------|-------------------|
| Phase tables | PHASE1-PHASE6, PHSPRDT1-6 | Alarm 1107 -> DOFSTART |
| FAILREG | FAILREG, +1, +2 | Overwritten with restart alarm code |
| Executive state | PRIORITY words (core sets) | Reinitialize via STARTSB2 |
| Waitlist | LST1, LST2 | Reinitialize to ENDTASK/NEG1/2 |
| General erasable | Random addresses 0-03777 octal | REDOCTR increments, no hang |

### Key Verification

- **REDOCTR** increments on every GOPROG (hardware restart) -- line 290 of FRESH_START_AND_RESTART.agc
- **Timeout detection**: 10s simulated time max per iteration
- **Deterministic seeds**: Fixed proptest seeds for CI, random for exploration

### User Implementation Opportunity

**`state_validator.rs`** -- Defining what "correct recovery" means for each corruption scenario requires detailed knowledge of the restart architecture. For example: after phase table corruption, should DOFSTART zero all flagwords, or only a subset? The answer is in lines ~200-300 of FRESH_START_AND_RESTART.agc.

**`coverage_tracker.rs`** -- Mapping yaAGC program counter values back to source lines using the `.lst` assembly listing file. The listing format is yaYUL-specific.

---

## Dependency Graph

```
Phase 1 (CI Pipeline)
  |
  +---> Phase 2 (Arithmetic Tests)     [can develop in parallel, CI integration needs Phase 1]
  |
  +---> Phase 3 (Integration via yaAGC) [requires Phase 1's .bin output + Docker image]
          |
          +---> Phase 4 (Restart Fuzzing)  [extends Phase 3's yaAGC infrastructure]
```

## Cargo Workspace Layout

```toml
# tests/Cargo.toml (workspace root)
[workspace]
members = ["agc-math", "agc-integration", "agc-fuzzer"]

[workspace.dependencies]
proptest = "1"
tokio = { version = "1", features = ["full"] }
```

## Implementation Sequence

1. **Phase 1** -- CI pipeline
   - `tools/build.sh`, `tools/verify-binary.sh`, `tools/binsource-to-bin.rs`
   - `tools/Dockerfile` (yaYUL + yaAGC + Rust toolchain)
   - `.github/workflows/agc-build.yml`
2. **Phase 2** -- Arithmetic tests (parallel with Phase 1)
   - `tests/agc-math/src/ones_complement.rs` (foundation -- AgcWord, DpWord, TpWord newtypes)
   - `tests/agc-math/src/constants.rs`, `tests/agc-math/src/poly.rs`
   - `tests/agc-math/src/arithmetic.rs` (DAD, DSU, DMP, DMPR)
   - User implements: DDV/BDDV, SQRTSUB
   - `tests/agc-math/src/trig.rs` (with exact Hastings coefficients)
   - `tests/agc-math/src/vector.rs`
   - All test modules
3. **Phase 3** -- Integration tests
   - `tests/agc-integration/src/` (DSKY client, AGC runner, protocol)
   - Basic scenarios (fresh start, verb/noun, lamp test)
   - User implements: P63/P64 transition test
4. **Phase 4** -- Fuzzing
   - Corruption strategies and injector
   - User implements: state validator, coverage tracker
   - Fuzz campaigns

## Verification Strategy

Each phase validates itself:

- **Phase 1**: Intentional `.agc` typo -> build failure; clean source -> binary match
- **Phase 2**: Hand-computed reference values (not from the Rust model) to catch circular bugs where model and tests agree on a wrong answer
- **Phase 3**: "Canary" test: invalid verb V99 -> OPR ERR, validates protocol independently of AGC logic
- **Phase 4**: Zero-corruption baseline establishes clean restart behavior

## Constraints

- **No `.agc` files modified** -- all new code in `/tests`, `/tools`, `/.github/workflows`, `/reference`
- **All arithmetic tests validate 15-bit 1's complement semantics** -- not IEEE 754
- **Rust newtypes enforce invariants at compile time** -- can't accidentally mix SP/DP/TP words
- **yaYUL/yaAGC from Virtual AGC project** -- pinned commit for reproducibility
