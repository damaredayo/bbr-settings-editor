# bbr-settings-editor

## An open source application for easily editing and distributing settings for Battlebit Remastered.

> 🛈 Please note that this project is not affiliated with Battlebit Remastered or its developers. This is a community project and is not officially supported by the developers of Battlebit Remastered.

Below you can find all of the currently supported features and the ones that are planned to be implemented. If you have any suggestions or ideas, please feel free to open an issue or a pull request. I would love to see them! :)

## Features / Todolist

- [x] Windows Registry support
- [ ] Full support for all settings
- [x] Support for most common settings
- [ ] Native Linux support (for now just run wine in the prefix)
- [x] Configuration file support (export/import)
- [x] Filters for settings
- [ ] GUI for editing settings
- [ ] Website for sharing settings

## Installing

### Prebuilt binaries

You can find builds in the [releases](https://github.com/damaredayo/bbr-settings-editor/releases) page.

## Usage

| Long name | Short name | Description | Type | Example |
| --------- | ---------- | ----------- | ---- | ------- |
| `--input` | `-i` | The filepath of the TOML to import | Filepath | `-i settings.toml`
| `--output` | `-o` | The filepath to export the TOML to | Filepath | `-o settings.toml`
| `--filters` | `-f` | Filters to include during an export | List | `-f common`
| `--help` | `-h` | Print help | Flag | `-h`
| `--version` | `-V` | Print version | Flag | `-V`

## Full example

```bash
## Export settings
bbr-settings-editor -o settings.toml -f common

## Import settings
bbr-settings-editor -i settings.toml
```

## Filters

You can use filters to only export specific settings. You can do this with the following syntax:

```bash
bbr-settings-editor -i settings.toml -o settings.toml -f hitmarkers -f keybindings
## OR
bbr-settings-editor -i settings.toml -o settings.toml -f hitmarkers,keybindings
```

The following filters are available:

- `common` (RECOMMENDED, includes the following: `hitmarkers`, `keybindings`, `audio`)
- `hitmarkers`
- `keybindings`
- `sentivity`
- `audio`

## Configuration file

The configuration file is a TOML file, the format is as follows:

```toml
[binding] # Name of the setting
type = "string" # Type of the setting
value = "F" # Value of the setting
```

## Building

bbr-settings-editor is written in Rust, so you will need to have Rust installed in order to build it. You can get it from [here](https://rustup.rs/).

After you have Rust installed, you can clone the repository and build the project by running the following command in the root directory of the project:

```bash
cargo build --release
```

Upon buidling, the binary will be located in `target/release/bbr-settings-editor`.

