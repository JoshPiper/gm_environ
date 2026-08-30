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

> [!WARNING]
> Installing the **client** realm module lets any server you join read your local environment variables through clientside Lua the server sends you — there's no prompt, and no way for you to tell it happened. `require("environ")` doesn't enumerate variables, but a script can still request specific ones by name (`USERNAME`, `COMPUTERNAME`, `PATH`, API keys, tokens, anything else you keep in your environment), and env vars often hold credentials. Only install `gmcl_environ` if you understand and accept that every server you join afterwards gets this ability. The server realm has no such exposure to other players — it only reaches the server's own environment.

On macOS, downloaded files carry the quarantine attribute and may be blocked from loading; clear it with `xattr -d com.apple.quarantine <file>`.

## Usage

```lua
require("environ")
-- Loads _G.environ

local home = environ.HOME -- "/home/gmod", or nil if it isn't set
local paths = environ.get_path() -- { "/usr/local/bin", "/usr/bin", ... }
```

An [LuaLS](https://github.com/LuaLS/lua-language-server) type definition file is available - see [Editor support](#editor-support) for autocomplete and inline docs while writing Lua against this module.

## Semantics

- `environ` is **read-only**. Assigning to any key raises a Lua error; a process can't usefully rewrite the environment it inherited, so the module doesn't pretend otherwise.

  ```lua
  environ.PATH = "/tmp" -- error: environ: environment variables cannot be set
  ```

- `environ` is a **userdata, not a table**. Every read goes through its metatable, and there's no way to enumerate it — `pairs(environ)` errors, and nothing here lists every variable. You can only ask for one by name.
- **Nothing raises for a missing variable.** An unset variable reads as `nil`, and the splitters return an empty table for one, so `pcall` isn't needed around any of this. The flip side is that "unset" and "set to an empty value" are indistinguishable through `get_path()` and `get_csv()`.
- **Values are read fresh on every access**, not captured when the module loads. In practice the environment a process inherited doesn't change while it runs, so this rarely matters.
- **Both call forms work.** `environ.get_csv("PATH")` and `environ:get_csv("PATH")` are equivalent — the module works out which argument slot the key landed in. The dot form is used throughout this document.
- **Names are reserved by case.** Lowercase names belong to the module: `get_path`, `get_csv` and the rest live there, and future releases may add more, so a lowercase environment variable isn't guaranteed to stay reachable by index. UPPERCASE names belong to the environment and the module will never claim one — `environ.PATH` is safe from any version of this module. Mixed-case names are reserved by neither; they resolve as environment variables today, but that isn't a promise.

## API Reference

### `environ.<NAME>: string`

Returns the value of the environment variable `<NAME>`, or `nil` if it isn't set. Any string key works, including names that aren't valid Lua identifiers — reach those with bracket syntax:

```lua
local home = environ.HOME
local pf = environ["ProgramFiles(x86)"]
```

A value that isn't valid UTF-8 also reads as `nil`, indistinguishable from an unset variable. This is only reachable on Linux and macOS, where the OS stores environment values as raw bytes rather than text.

`get_path`, `get_csv`, `get_version` and `get_build_info` are the names this doesn't apply to: they always resolve to the functions below, shadowing any environment variable that happens to share the name. `environ.get_csv("get_path")` reads the variable itself, if you ever need it.

### `environ.get_path(): table`

Returns `PATH` split into its component directories, using the host's separator (`;` on Windows, `:` everywhere else). Entries are trimmed and empty ones dropped, so the result is always a dense array of non-empty strings — safe to `ipairs` over or take `#` of. Returns an empty table if `PATH` isn't set; never raises.

```lua
for _, dir in ipairs(environ.get_path()) do
    print(dir) -- "/usr/local/bin", "/usr/bin", ...
end
```

The separator is fixed when the binary is built, so it matches the platform the module was compiled for.

### `environ.get_csv(key): table`

Returns the variable named `key` split on commas, trimmed and compacted the same way `get_path()` is. Returns an empty table if the variable isn't set — as it also does for one set to `""`, or to `",,,"`. Raises only if `key` isn't a string.

```lua
-- MY_ADDONS="foo, bar ,,baz"
local addons = environ.get_csv("MY_ADDONS") -- { "foo", "bar", "baz" }
```

Splitting is unconditional and has no escape syntax: this is a plain separated list, not [RFC 4180](https://www.rfc-editor.org/rfc/rfc4180) CSV, so a value containing a quoted comma splits on that comma too.

### `environ.get_version(): string`

Returns the module's own version, e.g. `"0.4.2"`. Never raises.

### `environ.get_build_info(): table`

Returns build information for the running binary:

```lua
{
    version       = "0.4.2",
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

`official` is a convenience for spotting a hand-built binary, not a security boundary — nothing stops a local build from setting the same environment variables CI does. [Verifying a release](#verifying-a-release) is the check that actually proves provenance.

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
