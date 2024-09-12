# gcode-thumb

Extracts the embedded thumbnail preview from gcode files into its own file. Intended to be used as a thumbnailer, for example for nautilus/kde.

## Usage

```bash
gcode-thumb some_model.gcode output.png
```

## Building

```bash
cargo build --release
```

## Installation

To use as a thumbnailer, install `gcode-thumb` to the `PATH` and copy `gcode.thumbnailer` to `/usr/share/thumbnailers/`.
