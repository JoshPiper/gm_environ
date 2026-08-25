# Contributing to gm_environ

## Purpose

This document exists to:
- Explain how commit messages drive versioning and releases.
- Set out what CI gates a PR on.
- Save you rediscovering the local build/lint/test setup by trial and error.

---

## Conventional Commits

release-plz reads commit messages to drive versioning, changelogs, and releases.

Format: `<type>[optional scope]: <description>`

Types:

| Type | Effect |
|---|---|
| `fix:` | Patch release (`x.y.Z`) |
| `feat:` | Minor release (`x.Y.0`) |
| `feat!:` or a `BREAKING CHANGE:` footer | Major release (`X.0.0`) |
| `chore:`, `build:`, `ci:`, `docs:`, `test:`, `refactor:`, `style:` | No release triggered on their own |

Examples:

```
fix: trim PATH entries before pushing them to Lua

feat: expose environ.get_csv()

feat!: return nil instead of raising for unset variables

BREAKING CHANGE: reading an unset variable no longer raises.
```

If a PR mixes several logical changes, split the commits by type rather than reaching for whichever tag sounds biggest. release-plz aggregates every commit into the changelog regardless, so there's no upside to lumping it all under one `feat:` — and smaller commits review better besides.

---

## What CI actually gates

`ci.yml` runs, on every PR:
- `cargo fmt --check`
- `cargo clippy -D warnings`, both realms
- The full 10-target build matrix
- [GLuaTest](https://github.com/CFC-Servers/GLuaTest) against real Garry's Mod server instances

All of it needs to pass. There's no merging around a red check.

---

## What you don't need to do

- Bump the version.
- Touch `CHANGELOG.md`.
- Create a tag.

release-plz does all three off your commit messages once your PR's on `main`. If you catch yourself editing the version field in `Cargo.toml`, stop — that's not your job any more, and it'll just conflict with the release PR release-plz maintains automatically. A maintainer merges that when it's time to ship.

---

## Local setup

The pinned nightly toolchain in [`rust-toolchain.toml`](rust-toolchain.toml) gets picked up by `rustup` automatically — nothing to install by hand beyond `rustup` itself.

```bash
cargo build --release                    # server realm
cargo build --release --features gmcl    # client realm
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --features gmcl -- -D warnings
```

Run both clippy invocations, not just one — the two realms compile different code paths (see the `gmcl` feature gate in `src/lib.rs`), and CI lints both.

Note that `debug_print!` and friends compile away entirely in `--release`, so a binding used only inside one reads as unused there. That's why a few are named `_err`; the release build is expected to be warning-clean too, not just the debug one.

Cross-compiling to a specific release target needs that target installed:

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

32-bit Linux targets also need a multilib GCC (`sudo apt-get install gcc-multilib` on Debian/Ubuntu).

### Running the Lua tests locally

CI runs [GLuaTest](https://github.com/CFC-Servers/GLuaTest) against the built server module on every PR (`.github/workflows/ci.yml`). Running it locally needs Docker — see GLuaTest's own docs for the local invocation. Specs live in `lua/tests/environ/`.

---

## Code style

- No comments explaining *what* the code does — the names already do that. A comment earns its place only when it captures a non-obvious *why*: a hidden constraint, a workaround, a decision that would otherwise look like a mistake to a reviewer.
- Keep the `unsafe` surface exactly as small as it is today. Adding a new Lua-exported function? Follow the existing `#[lua_function]` pattern in `src/lib.rs` rather than inventing a new calling convention.
- A new function that reads an argument off the stack goes through the existing `requested_index!` macro, so it accepts both `environ.f("KEY")` and `environ:f("KEY")` like its neighbours do.
- Run `cargo fmt` before you push. CI enforces it, and it will not fix it for you.

---

## Reporting a security issue

Don't put it in a PR or an issue. See [SECURITY.md](SECURITY.md).
