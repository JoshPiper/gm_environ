--- @meta

-- Type definitions for gm_environ, for use with the Lua Language Server
-- (https://github.com/LuaLS/lua-language-server). This file declares the
-- shape of the API only; it has no runtime behaviour and must never be
-- require()'d in-game. Point your editor at it instead -- see the
-- "Editor support" section of the README.

--- The environment of the running game process, exposed read-only. Any
--- string key is looked up as an environment variable and comes back as a
--- string, or nil if that variable isn't set:
---
--- ```lua
--- local home = environ.HOME -- "/home/gmod", or nil
--- ```
---
--- A value that isn't valid UTF-8 reads as nil too, indistinguishable from
--- an unset variable. get_path and get_csv are the two names this doesn't
--- apply to: they always resolve to the functions below, shadowing any
--- variable of the same name (get_csv reads such a variable, if needed).
---
--- Assigning to any key raises a Lua error -- environment variables cannot
--- be set from Lua. It's a userdata rather than a table, so it can't be
--- iterated with pairs() either: there is no way to list every variable,
--- only to ask for one by name.
--- @class environ
--- @field [string] string? # nil if the variable isn't set
environ = {}

-- Both call forms reach the same code -- the module works out whether the
-- key landed in slot 1 (a dot call) or slot 2 (a colon call) -- so each
-- function below carries a colon-call overload alongside the dot-call
-- signature it's declared with. Prefer the dot form.

--- Returns PATH split into its component directories, using the host's
--- separator (";" on Windows, ":" everywhere else). Entries are trimmed and
--- empty ones dropped, so the result is always a dense array of non-empty
--- strings. Returns an empty table (never raises) if PATH isn't set.
--- @return string[]
--- @overload fun(self: environ): string[]
function environ.get_path() end

--- Returns the named variable split on commas, with each entry trimmed and
--- empty ones dropped. Returns an empty table if the variable isn't set --
--- indistinguishable from one set to "" or to ",,,". Raises only if key
--- isn't a string.
--- @param key string
--- @return string[]
--- @overload fun(self: environ, key: string): string[]
function environ.get_csv(key) end
