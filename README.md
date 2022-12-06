# zono

Zono is an open source, multi-instance launcher for Minecraft.

## Features

Zono's feature set is very incomplete as it is still in the very early stages of development. A summary of features desired for release is below.

- Support for multiple instances
- Modding support
	- Fabric
	- Forge
	- Quilt
- Mod registry support
	- CurseForge
	- Modrinth

## Quick Start

Since Zono is still under development, the quick-start is perhaps not as quick as it could be. Running Zono effectively requires a build from source.

### Build Dependencies

Here is a list of dependencies required to build Zono from source.

- Rust v1.65 (stable)
- NodeJS v16 or above
- PNPM v7

## Set-up

With the above dependencies installed, run the following commands:

```
pnpm i
```
```
pnpm tauri build
```

## Contributing

If you are interested in contributing to Zono, have a look at [CONTRIBUTING](./CONTRIBUTING.md). It provides some basic information on how the project is structured and how to contribute.

## License 

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fkaylendog%2Fzono.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fkaylendog%2Fzono?ref=badge_shield)

Zono is distributed under the terms of the GNU General Public License, a copy of which can be found in the [LICENSE](./LICENSE) file. More information on copyright terms can be found in [COPYRIGHT](./COPYRIGHT.md).
