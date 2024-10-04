# Tiny Pinch

Tiny Pinch is a modloader for Tiny Glade, allowing users to inject and run custom mods in the game.

⚠️ Back up any saves you have before experimenting ⚠️

## Components

Tiny Pinch consists of several components:

1. `tiny-pinch`: CLI interface for injecting mods into Tiny Glade
2. `canopy`: Modding framework for creating mods compatible with Tiny Pinch
3. `lumberjack-dumper`: Utility for extracting Tiny Glade offset and type information
4. `lumberjack`: Library for accessing dumps produced by lumberjack-dumper

## Prerequisites

- Rust nightly toolchain (for compiling Tiny Pinch)
- Rust version 1.78.0 (for compiling mods)
- [Rustup](https://rustup.rs/) (for installing Rust)

## Installation

### Install Tiny Pinch

Choose one of the following methods:

a. Install to PATH:

```bash
cargo install --path tiny-pinch
```

b. Compile the binary:

```bash
cargo build --release --package tiny-pinch
```

## Usage

Basic usage:

```bash
tiny-pinch <path/to/mod> [-- <additional_arguments>...]
```

For help:

```bash
tiny-pinch --help
```

Output:

```
Tiny Glade mod loader

Usage: tiny-pinch <MOD_PATH> [-- <ADDITIONAL_ARGUMENTS>...]

Arguments:
  <MOD_PATH>                 Path to the mod
  [ADDITIONAL_ARGUMENTS]...  Additional arguments passed to the mod

Options:
  -h, --help  Print help
```

## Running Lumberjack

Lumberjack-dumper extracts essential game information for mod development.

1. Compile lumberjack-dumper:

   ```bash
   cargo +1.78.0 build --release --package lumberjack-dumper
   ```

2. Run the dumper:
   ```bash
   tiny-pinch <path/to/lumberjack-dumper.dll>
   ```

Note: The game will launch, dump information, and close. An error message is expected and can be ignored.

## Example Mod

To run the example mod that counts critters in the world:

1. Compile the mod:

   ```bash
   cd examples/simple_mod
   cargo +1.78.0 build --release
   ```

2. Inject the mod:
   ```bash
   tiny-pinch <path/to/examples/simple_mod/target/release/simple_mod>
   ```

## Contributing

Contributions are welcome! Here are some areas where help is needed:

- Documentation improvements
- Enhancing the Canopy API
- Addressing compatibility issues with Bevy ecosystem plugins

### Known Issues & Disclaimers

- Arbitrary Bevy ecosystem plugins may not function properly due to schedule location methods in Canopy.
- Potential solution: Implement a `canopy_add_plugins` method to add plugins to a separate app and merge with the main app.
- This currently only works on Windows
