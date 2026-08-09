# aseprite-extension-with-rust

An Aseprite extension template backed by a native Rust library, loaded into Aseprite's Lua scripting environment via `package.loadlib`.

## Prerequisites

- **Rust**: install via [rustup.rs](https://rustup.rs)
- **A C compiler**: `build.rs` compiles the vendored Lua sources with the `cc` crate, so this is required even though no Rust code depends on it directly.

## Getting the source

This repo pulls Aseprite's own Lua fork in as the `lua-src` git submodule.
Clone recursively so it's fetched too:

```bash
git clone --recursive <repo-url>
```

Or if you already cloned without `--recursive`:

```bash
git submodule update --init --recursive
```

## Building

```bash
cargo build            # debug
cargo build --release  # release
```

`build.rs` compiles `lua-src/*.c` and statically links the result into the extension's native library, there's no separate `liblua*.so`/`.dll` to ship or load.

## Packaging

Building alone doesn't produce something Aseprite can install, use the scripts in [tools/](tools):

```bash
tools/zip.sh   # Linux/macOS
tools/zip.cmd  # Windows
```

Each does a release build, copies the resulting library into `lua/`, and zips `lua/` into `dist/ext.zip`. Install that zip in Aseprite via **Edit > Preferences > Extensions > Add Extension**.
