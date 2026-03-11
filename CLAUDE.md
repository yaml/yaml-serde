# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`yaml-serde` is the officially maintained fork of `serde_yaml`, bridging Rust's
Serde serialization framework with YAML.
It is published under the YAML organization and uses `libyaml-rs` for
low-level YAML parsing and emission.

## Common Commands

All commands use the Makes system (dependencies installed under `.cache/`).

```bash
make shell cmd='cargo build'
make shell cmd='cargo test'
make shell cmd='cargo test test_name'          # Run a single test
make shell cmd='cargo clippy --tests -- -Dclippy::all -Dclippy::pedantic'
make shell cmd='cargo doc'
```

Clippy is run with pedantic lints enabled and `-Dwarnings` is implicit in CI.
Fix all Clippy warnings before committing.

## Architecture

The library has two main layers:

**High-level API (`lib.rs`):** Exports `from_str`, `from_slice`, `from_reader`,
`to_string`, `to_writer`, and Value-based conversions (`from_value`,
`to_value`).

**Core modules:**

- `de.rs` — Serde `Deserializer` impl; drives `loader.rs` to consume YAML
  events and feeds them into Serde visitors.
  Handles anchors/aliases, enum tags (`!TagName`), and multi-document YAML.
- `ser.rs` — Serde `Serializer` impl; uses `libyaml/emitter.rs` to write YAML.
  Determines scalar style (plain, quoted, literal) based on content.
- `loader.rs` — Wraps the libyaml parser into a `Document` (event list +
  alias map); called by `de.rs`.
- `value/` — The loosely-typed `Value` enum (`Null`, `Bool`, `Number`,
  `String`, `Sequence`, `Mapping`, `Tagged`) with its own Serialize/Deserialize
  impls in `value/ser.rs` and `value/de.rs`.
- `mapping.rs` — `Mapping` type backed by `IndexMap<Value, Value>` (preserves
  insertion order).
- `number.rs` — `Number` with internal `PosInt(u64)`, `NegInt(i64)`,
  `Float(f64)` variants.
- `libyaml/` — Thin safe wrappers around `libyaml-rs` FFI: `parser.rs`,
  `emitter.rs`, `tag.rs`, `error.rs`.
- `error.rs` — `Error` type with `Location` (line/column/index).
- `with.rs` — Helpers for `#[serde(with = "...")]` attributes.

**Data flow (deserialization):**

```
input bytes/str/reader
  → loader::Loader (libyaml parser)
  → loader::Document (event list)
  → de::Deserializer (Serde visitor dispatch)
  → user's Rust type
```

**Data flow (serialization):**

```
user's Rust type
  → ser::Serializer (Serde serializer)
  → libyaml::emitter::Emitter
  → YAML bytes/string
```

## Key Dependencies

- `serde` — serialization framework
- `libyaml-rs` — C YAML parser/emitter, safe wrapper
- `indexmap` — ordered map for `Mapping`

## Tests

Tests live in `tests/`:

- `test_de.rs` — deserialization edge cases
- `test_ser.rs` / `test_serde.rs` — serialization and round-trip
- `test_error.rs` — error messages and locations
- `test_value.rs` — Value API

Fuzz targets are in `fuzz/`.
CI also runs Miri (with `-Zmiri-strict-provenance`) and minimal dependency
versions.
