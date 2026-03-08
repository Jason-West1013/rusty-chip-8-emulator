# Rusty CHIP-8 Emulator

A CHIP-8 emulator written in Rust as a learning project to explore systems programming and emulation fundamentals.

## About

CHIP-8 is an interpreted programming language developed in the 1970s, designed to make game development easier for early microcomputers. This emulator implements the full CHIP-8 specification including its 35 opcodes, 64×32 monochrome display, 16-key hex keypad, and dual timer system.

## Built With

- [Rust](https://www.rust-lang.org/)
- [pixels](https://github.com/parasyte/pixels) — framebuffer rendering
- [winit](https://github.com/rust-windowing/winit) — window management and input

## Running the Emulator

```bash
cargo run
```

## References

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Tobias V. Langhoff's CHIP-8 Guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite)