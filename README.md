# Tiny Pinch

Tiny Glade modloader.

**Please back up any saves you have before experimenting with anything here**

## What's Here

1. `tiny-pinch`: CLI interface for injecting mods into Tiny Glade
1. `canopy`: modding framework for creating mods injected by Tiny Pinch
1. `lumberjack-dumper`: tiny glade offset / type informaton dumper. These dumps are leveraged by canopy mods to create more user friendly Bevy code.
1. `lumberjack`: small crate for accessing dumps produced by lumberjack-dumper

## Getting Started


### Prerequisites

Nightly toolchain is required to compile Tiny Pinch, and rust version 1.78.0 is required to compile mods.

### Install Tiny Pinch

Tiny Pinch is required to inject mods into Tiny Glade currently. To install it you can run `cargo install --path tiny-pinch` which will install it to path, or `cargo build --release --package tiny-pinch` which will compile the binary.


#### Tiny Pinch Usage

The basic usage of `tiny-pinch` is simply `tiny-pinch <path/to/mod>` but additional options can be passed to the mod with `--`.

```
> tiny-pinch --help
Tiny Glade mod loader

Usage: tiny-pinch <MOD_PATH> [-- <ADDITIONAL_ARGUMENTS>...]

Arguments:
  <MOD_PATH>                 Path to the mod
  [ADDITIONAL_ARGUMENTS]...  Additional arguments passed to the mod

Options:
  -h, --help  Print help
```

### Run Lumberjack

Compile `lumberjack-dumper` by running `cargo +1.78.0 build --release lumberjack-dumper`. Make sure to compile it under 1.78.0 or you will difficult to understand errors. Then run the dumper with `tiny-pinch <path/to/lumberjack-dumper.dll>`. This will launch the game, dump the information, then close the game. You will get an error, but that is to be expected.

### Run Example Mod

These steps are largely the same as the previous section, compile the example mod in `examples/simple_mod` and inject it. This mod just counts the number of critters in the world and outputs it.

## Contributing

Help is always appreciated. Any documentation improvements are welcome, as well as working on the canopy api. A fork of bevy might be necessary eventually.

The issues I see being major in the long run are:

- Arbitrary bevy ecosystem plugins not functioning properly.
    - Since schedules are located by strings currently in canopy (`canopy_add_systems("Update", ...)`) the bevy ecosystem is incompatible with canopy mods.
    - A possible solution could be to have a `canopy_add_plugins` method which adds all the plugins to a separate app then merges it with the main app.
