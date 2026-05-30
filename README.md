# eiipm

**Eiipm** pronounced as **ee-pee-em** is a plugin manager built for [Ewwii](https://ewwii-sh.github.io) (Elkowar's Wacky Widgets Improved Interface).

## Installation

Download the latest release from:

https://github.com/Ewwii-sh/eiipm/releases/latest/

or build from source:

```bash
git clone https://github.com/Ewwii-sh/eiipm
cd eiipm
cargo build --release
# copy target/release/eiipm somewhere on your PATH
```

## Usage

### Initialization

Go to your ewwii configuration directory and run the following command to initialize the plugin manager for that configuration.

```bash
$ eiipm init
```

This will create these files/directories: `plugins.toml`, `plugins.lock`, and `plugins/`.

### Other Commands

```bash
# Add plugins
eiipm add user/repo
eiipm add user/repo --ref v1.2.0  # pin to a tag
eiipm add user/repo --prebuilt    # prefer prebuilt binary if available
eiipm add user/repo --build "cargo build --release" --artifact "target/release/libmy-plugin.so"

# Install plugins
eiipm install

# Update Plugins
eiipm update           # update all plugins
eiipm update user/repo # update only this plugin

# Remove plugins
eiipm remove user/repo

# List plugins
eiipm list

# Cleaning
eiipm clean        # remove untracked artifacts from 'plugins/'
eiipm cache-clean  # wipe the global source cache (~/.cache/eiipm/)
```

## Editing `plugins.toml`

```toml
[plugins]
# shorthand (it can be a branch, tag, or SHA)
"user/repo" = "main"
"user/repo2" = "v1.2.0"

# full config (override build command, artifact path, or prefer prebuilt)
"user/repo3" = { ref = "main", prebuilt = true }
"user/repo4" = { ref = "main", build = "make release", artifact = "build/out.so" }
```

## Authoring Plugins

If you are writing a plugin for ewwii and want it to be compatible with eiipm, then add a `plugin.toml` file at the root of your repository:

```toml
[plugin]
build = "cargo build --release"
artifact = "target/release/libmyplugin.so"

# optional: provide a prebuilt binary for users who don't want to build
[plugin.prebuilt]
url = "https://github.com/user/repo/releases/download/{version}/libmyplugin-{arch}-{os}.so"
```
