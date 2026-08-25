# gm_environ

[![CI](https://github.com/JoshPiper/gm_environ/actions/workflows/ci.yml/badge.svg)](https://github.com/JoshPiper/gm_environ/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/JoshPiper/gm_environ)](https://github.com/JoshPiper/gm_environ/releases/latest)
[![License: MIT](https://img.shields.io/github/license/JoshPiper/gm_environ)](LICENSE)

Using Environment Variables in Garry's Mod.

## Installation

Download a copy of the module from the [releases](https://github.com/JoshPiper/gm_environ/releases/latest) page (or [compile from source](#building-from-source)), and move it to:

```
<Garry's Mod Installation>/garrysmod/lua/bin/<filename>
```

`<filename>` follows the pattern `gm<realm>_environ_<platform>.dll`.

| Realm | Platform | Filename |
|---|---|---|
| Server | Windows, 32-bit (`main` branch) | `gmsv_environ_win32.dll` |
| Server | Windows, 64-bit (`x86-64` branch) | `gmsv_environ_win64.dll` |
| Server | Linux, 32-bit (`main` branch) | `gmsv_environ_linux.dll` |
| Server | Linux, 64-bit (`x86-64` branch) | `gmsv_environ_linux64.dll` |
| Server | macOS, 64-bit (`x86-64` branch) | `gmsv_environ_osx64.dll` |
| Client | Windows, 32-bit | `gmcl_environ_win32.dll` |
| Client | Windows, 64-bit | `gmcl_environ_win64.dll` |
| Client | Linux, 32-bit | `gmcl_environ_linux.dll` |
| Client | Linux, 64-bit | `gmcl_environ_linux64.dll` |
| Client | macOS, 64-bit | `gmcl_environ_osx64.dll` |

Server realm exposes the module to server-side Lua; client realm exposes it to the client console/menu. Install whichever (or both) your use case needs.

On macOS, downloaded files carry the quarantine attribute and may be blocked from loading; clear it with `xattr -d com.apple.quarantine <file>`.

## Usage

```lua
require("environ")
-- Loads _G.environ

-- NYI
```

An [LuaLS](https://github.com/LuaLS/lua-language-server) type definition file is available - see [Editor support](#editor-support) for autocomplete and inline docs while writing Lua against this module.

## API Reference

NYI

## Editor support

A [LuaLS](https://github.com/LuaLS/lua-language-server) type definition file, [`environ.lua`](environ.lua), ships in this repository and as a release asset. It's declarations only (`---@meta`) — never `require()` it in-game. Point your editor at it instead, e.g. in `.luarc.json`:

```json
{
    "workspace.library": ["path/to/environ.lua"]
}
```

## Building from source

Requires the Rust nightly toolchain pinned in [`rust-toolchain.toml`](rust-toolchain.toml), which `rustup` will automatically install on first build.

```bash
git clone https://github.com/JoshPiper/gm_environ
cd gm_environ
cargo build --release                    # server realm -> target/release/{lib,}environ.{so,dll,dylib}
cargo build --release --features gmcl    # client realm
```

Cross-compiling to another target needs that target installed (`rustup target add <triple>`) and, for 32-bit Linux specifically, a multilib GCC (`gcc-multilib` on Debian/Ubuntu).
See [`.github/workflows/build.yml`](.github/workflows/build.yml) for the exact target/feature matrix CI builds.
There are some reported pitfalls in cross compiling GMod binaries, so take care during the process.

## Verifying a release

Every release binary is built by this repository's GitHub Actions workflow and cryptographically attested. To verify a downloaded file actually came from that pipeline (requires the [GitHub CLI](https://cli.github.com/)):

```bash
gh attestation verify gmsv_environ_linux64.dll -R JoshPiper/gm_environ
```

Each release also ships:
- **`SHA256SUMS`** - checksums for every asset in the release.
- **`gm_environ-<version>.cdx.json`** - a [CycloneDX](https://cyclonedx.org/) software bill of materials for the dependency tree at that version.

Additionally, every binary is built with [`cargo-auditable`](https://github.com/rust-secure-code/cargo-auditable): the exact dependency versions that went into it are embedded in the file itself, so it can be scanned for known vulnerabilities on its own, without needing the SBOM asset or the source repository:

```bash
cargo install cargo-audit --features=binary-scanning
cargo audit bin gmsv_environ_linux64.dll
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

See [SECURITY.md](SECURITY.md) for how to report a vulnerability.

## Credits

Massive thanks to [Billy](https://github.com/WilliamVenner) for [gmod-rs](https://github.com/WilliamVenner/gmod-rs).
