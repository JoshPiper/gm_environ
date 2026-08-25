--- @meta

-- Type definitions for gm_environ, for use with the Lua Language Server
-- (https://github.com/LuaLS/lua-language-server). This file declares the
-- shape of the API only; it has no runtime behaviour and must never be
-- require()'d in-game. Point your editor at it instead -- see the
-- "Editor support" section of the README.

--- The environment of the host process, exposed as a read-only userdata.
---
--- Any key that isn't one of the functions below is looked up as an
--- environment variable: `environ.HOME` is the value of `$HOME`, or `nil` if
--- it isn't set. Assigning to any key raises.
---
--- Every function here accepts both the dot and the colon call form --
--- `environ.get_csv("X")` and `environ:get_csv("X")` are equivalent. The
--- docs use the dot form throughout.
---
--- @class environ
--- @field [string] string? # The named environment variable, or nil if unset.
environ = {}

--- Returns `PATH` split on the platform's path separator (`:` on Unix, `;`
--- on Windows), with entries trimmed and blanks dropped. Never raises --
--- returns an empty table if `PATH` is unset.
--- @return string[]
function environ.get_path() end

--- Returns the named environment variable split on commas, with entries
--- trimmed and blanks dropped. Never raises -- returns an empty table if the
--- variable is unset.
--- @param key string # Name of the environment variable to read.
--- @return string[]
function environ.get_csv(key) end

--- Returns the module's own version, e.g. "0.4.1".
--- @return string
function environ.get_version() end

--- @class EnvironBuildInfo
--- @field version string
--- @field commit string? # nil if not built from a git checkout
--- @field commit_short string? # nil if not built from a git checkout
--- @field dirty boolean? # nil if unknown
--- @field built_at string
--- @field target string
--- @field realm "sv"|"cl"
--- @field rustc_version string
--- @field official boolean # true only for binaries built by this project's CI
--- @field repository string # empty outside of GitHub Actions
--- @field run_url string # empty outside of GitHub Actions

--- Returns build provenance for the running binary. See the README's
--- "Verifying a release" section before trusting `official`, `commit`,
--- or `run_url` for anything security-sensitive.
--- @return EnvironBuildInfo
function environ.get_build_info() end
