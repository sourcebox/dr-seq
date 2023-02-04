# dr-seq

Grid-based drum sequencer plugin as MIDI FX in CLAP/VST3 format.

**WARNING:** This project is in a very early state. So there is no guarantee for anything and it may not even build. It may even disappear completely.

**Don't file any issues or PRs! It's not the time for it yet.**

## Some Notes

- The GUI causes high cpu load if opened. This is under investigation and will be hopefully improved soon.
- Expect a lot of incompatible changes in the future. Don't expect that any settings will be reloaded as expected by a newer version.
- Testing is currently only done using Bitwig Studio on Linux.
- Ableton Live will refuse to load this plugin because it doesn't support external MIDI effects.

## Building

- `cargo run` runs the standalone application.
- `cargo bundle` creates the plugin bundles in `target/bundled`.

Add `--release` to the cargo commands for an optimized build.

## License

All parts of this project are published under the MIT license, except the VST3 crate, which is published under the GPL3 license to comply with the requirements of the VST3 SDK. All contributions to this project must be provided under the same license conditions.

Author: Oliver Rockstedt <info@sourcebox.de>
