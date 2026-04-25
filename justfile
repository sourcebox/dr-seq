# Default, just list available receipes.
_default:
    @just --list

# Run the standalone.
run flags="":
    cargo run {{ flags }}

# Bundle the plugins.
bundle flags="":
    cargo xtask bundle dr-seq-vst3 {{ flags }}
