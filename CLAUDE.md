## Project
CHIP-8 emulator written in Rust. Primary goal is learning Rust and low-level programming concepts — prioritize clarity and idiomatic Rust over cleverness.

## Commands
- Build: `cargo build`
- Test: `cargo test`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`

## Constraints
- Don't hide low-level details behind heavy abstractions — I want to see how things work
- Explain *why* before suggesting refactors
- Flag when something I'm doing diverges from CHIP-8 spec

## General Architecture
- Using the winit and pixels packages from display

## CPU Architecture
- Loading the font data at 0x050..0x0A0 to handle ROMs that load their data at the very start of memory
- Combining the decode and execute steps for the fetch, decode, and execute methods