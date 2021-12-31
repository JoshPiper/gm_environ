# [0.3.0](https://github.com/JoshPiper/gm_environ/compare/v0.2.2...v0.3.0) (2021-12-31)


### Bug Fixes

* Allow error to operate on any type of string. ([89b91f5](https://github.com/JoshPiper/gm_environ/commit/89b91f5960cebfbd3ce9763605619cd52a6f05ad))
* ensure that parts of PATH are trimmed before being sent. ([5bb2973](https://github.com/JoshPiper/gm_environ/commit/5bb2973dc330fa8b04e14358783155f6271d9c7d))
* Increase default memory for both environ and environ.__metatable. ([594d7e9](https://github.com/JoshPiper/gm_environ/commit/594d7e914478fc8d84db0805d89dbe16b052dc35))
* Replace println! with debug_println! ([12b3a2b](https://github.com/JoshPiper/gm_environ/commit/12b3a2b3df0ece3e9ec8c37e61655c78ae55c1a5))


### Features

* add environ.get_csv, for comma seperated environment vars. ([557be0f](https://github.com/JoshPiper/gm_environ/commit/557be0ffcae811f01e7a788d1d2f2dd72751302f))
* add newindex to the metatable to prevent setting random values on the environ table. ([d2d1b00](https://github.com/JoshPiper/gm_environ/commit/d2d1b00a21104f6c822fb1027f3d203f30cb5b71))
* add requested_index macro, to abstract out env key fetching. ([d7eae99](https://github.com/JoshPiper/gm_environ/commit/d7eae99c97998794c45ef1336f0b2ce7598542d2))
* use requested_index! macro for __index. ([5d314be](https://github.com/JoshPiper/gm_environ/commit/5d314be5d2e5c68128ce859fc18e1bff2af63b6a))



## [0.2.2](https://github.com/JoshPiper/gm_environ/compare/v0.2.1...v0.2.2) (2021-12-31)


### Bug Fixes

* Implement build fixes from gm_sysinfo. ([a50c9d4](https://github.com/JoshPiper/gm_environ/commit/a50c9d427a458e7e023600f80afc5b89fbef1069))



## [0.2.1](https://github.com/JoshPiper/gm_environ/compare/v0.2.0...v0.2.1) (2021-12-30)


### Bug Fixes

* Fixes environ vs sysinfo in build script. ([92ac0f7](https://github.com/JoshPiper/gm_environ/commit/92ac0f7c3d2858643219f4a0377626b672d4c64e))



# [0.2.0](https://github.com/JoshPiper/gm_environ/compare/v0.1.0...v0.2.0) (2021-12-30)


### Bug Fixes

* call LuaState:error directly. ([17ecfbd](https://github.com/JoshPiper/gm_environ/commit/17ecfbd0211b6615bb6778c6b995bc617e406c22))
* fetch all commits. ([e4c68d8](https://github.com/JoshPiper/gm_environ/commit/e4c68d8d5674ce06690dd1ff75dfdd82f695754a))
* update to latest gmod version to fix crashing issue. ([cbf539a](https://github.com/JoshPiper/gm_environ/commit/cbf539a05a7adda3f073978e0cdc25cd33b25796))
* we're no longer borrowing. ([376264a](https://github.com/JoshPiper/gm_environ/commit/376264af3c9dfd41b4e913f70aae5d558ca584f6))


### Features

* add path seperators. ([78670c0](https://github.com/JoshPiper/gm_environ/commit/78670c00c5f6678bb245a65771a1ffd325ee55c5))



# [0.1.0](https://github.com/JoshPiper/gm_environ/compare/debd60ba289d98bdee6c37baae4a2b227bbe638f...v0.1.0) (2021-12-30)


### Features

* update to latest wip. ([debd60b](https://github.com/JoshPiper/gm_environ/commit/debd60ba289d98bdee6c37baae4a2b227bbe638f))



