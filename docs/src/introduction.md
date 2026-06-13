# Introduction

Eiipm is a fast and eligant plugin manager made in rust for [Ewwii](https://github.com/Ewwii-sh/ewwii). It helps automate and manage your ewwii plugins with ease!

## Installation

Download the latest release from:

[https://github.com/Ewwii-sh/eiipm/releases/latest/](https://github.com/Ewwii-sh/eiipm/releases/latest/)

or build from source:

```bash
git clone https://github.com/Ewwii-sh/eiipm
cd eiipm
cargo build --release
# copy target/release/eiipm somewhere on your PATH
```

## Basics

Ewwii supports plugins which allow extending the core functionality of the ewwii engine. The plugins go into the `plugins/` directory inside an ewwii configuration. For example, let's say you have an ewwii configuration at `~/.config/ewwii/`. To add and use a plugin, you have to create a `plugins/` directory inside `~/.config/ewwii/` and place the plugin there.

```
~/.config/ewwii/plugins/
  |- libxyz.so 
  |- libabc.so
```

Although this is straightforward, things can get messy quickly when you have a lot plugins. This is where eiipm comes in. Just run `eiipm init` in the `~/.config/ewwii/` directory and the basic plugin manager setup, including the `plugins/` directory will be automatically created. After this, all the plugins can be maintained either by editing the `plugins.toml` file or using the eiipm commands shown in later sections.

Even if you don't use many plugins, eiipm can still be useful as it automates downloading and compiling the plugins.
