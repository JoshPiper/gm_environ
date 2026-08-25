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

local home = environ.HOME -- "/home/gmod", or nil if unset
local dirs = environ.get_path() -- { "/usr/local/bin", "/usr/bin", ... }
```

A [LuaLS](https://github.com/LuaLS/lua-language-server) type definition file is available - see [Editor support](#editor-support) for autocomplete and inline docs while writing Lua against this module.

## Semantics

- `environ` is a **userdata**, not a table. Reads go through an `__index` metamethod that hits the process environment on every lookup, so a variable changed by something else in the process is visible immediately - there is no snapshot taken at load time. The flip side is that it does not behave like a table: `pairs()`, `#`, and `table.*` will not work on it, and there is no way to enumerate the environment.
- **Reads never raise.** An unset variable reads as `nil`, and the two splitting helpers return an empty table rather than erroring. This is the opposite of [gm_sysinfo](https://github.com/JoshPiper/gm_sysinfo)'s convention, deliberately: "this variable isn't set" is an ordinary, expected answer here, not a failure to report.
- **Writes always raise.** The module is strictly read-only; see [`__newindex`](#environkey--value) below.
- Every function accepts **both call forms**. `environ.get_csv("X")` and `environ:get_csv("X")` are equivalent - the key lands in a different argument slot and the module handles either. The docs below use the dot form throughout.
- Function names **shadow** environment variables. `environ.get_path` is always the function, even on a host that happens to export a variable called `get_path`. The shadowed names are `get_path`, `get_csv`, `get_version` and `get_build_info`.

## API Reference

### `environ.<KEY>: string?`
Returns the value of the `<KEY>` environment variable, or `nil` if it isn't set. Never raises.

```lua
local port = tonumber(environ.SRCDS_PORT) or 27015
```

Lookups are case-sensitive on Linux and macOS, and case-insensitive on Windows - this module doesn't normalise either way, it just asks the OS.

### `environ.get_path(): string[]`
Returns `PATH` split on the platform's path separator (`:` on Linux and macOS, `;` on Windows), as an array of strings. Entries are trimmed, and blanks are dropped, so a trailing separator or an empty component won't show up as an empty string in the result. Never raises - an unset `PATH` yields an empty table.

```lua
for _, dir in ipairs(environ.get_path()) do
    print(dir)
end
```

### `environ.get_csv(key): string[]`
Returns the named environment variable split on commas, with the same trimming and blank-dropping as `get_path()`. Never raises - an unset variable yields an empty table.

```lua
local admins = environ.get_csv("GMOD_ADMIN_STEAMIDS")
```

Note that an unset variable and a variable set to an empty string are indistinguishable through this function: both give `{}`. Read the raw value if you need to tell them apart.

### `environ.get_version(): string`
Returns the module's own version, e.g. `"0.4.1"`.

### `environ.get_build_info(): table`
Returns build information for the running binary:

```lua
{
    version       = "0.4.1",
    commit        = "c73b33dc3ad16535a344cd427909c77b79b60bef", -- or nil
    commit_short  = "c73b33d",                                   -- or nil
    dirty         = false,                                       -- or nil if unknown
    built_at      = "Thu, 13 Aug 2026 04:03:15 +0000",
    target        = "x86_64-unknown-linux-gnu",
    realm         = "sv",                                        -- or "cl"
    rustc_version = "rustc 1.99.0-nightly (ad3d0bc14 2026-07-31)",
    official      = true,  -- built by CI, not a local `cargo build`
    repository    = "JoshPiper/gm_environ",
    run_url       = "https://github.com/JoshPiper/gm_environ/actions/runs/123456",
}
```

`commit`, `commit_short`, and `dirty` are `nil` if the binary wasn't built from a git checkout. `repository` and `run_url` are empty strings outside of GitHub Actions. If `official` is `false`, or `run_url` doesn't resolve to a real workflow run, treat the binary as unverified; it wasn't built by this project's release pipeline.

### `environ.<KEY> = value`
**Always raises.** The environment is read-only through this module:

```lua
environ.PATH = "/tmp" -- error: environ: environment variables cannot be set
```

Read-only is a deliberate contract, not a missing feature, and it isn't likely to be relaxed: `setenv` is not thread-safe (Rust made `std::env::set_var` `unsafe` in the 2024 edition for exactly this reason), and Garry's Mod runs Lua alongside engine threads that read the environment, so a write from Lua could tear a read anywhere else in the process. If you need a value to travel from Lua outwards, use a file or a network call, not the environment.

## Editor support

A [LuaLS](https://github.com/LuaLS/lua-language-server) type definition file, [`environ.lua`](environ.lua), ships in this repository and as a release asset. It's declarations only (`---@meta`) - never `require()` it in-game. Point your editor at it instead, e.g. in `.luarc.json`:

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
