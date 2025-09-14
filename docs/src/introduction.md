# Introduction

Eiipm is a fast and eligant package manager made in rust for [Ewwii](https://github.com/Ewwii-sh/ewwii). Eiipm uses the package metadata from the [Ewwii-sh/eii-manifests](https://github.com/Ewwii-sh/eii-manifests) repository where the manifests of packages are stored.

## Installation

Checkout [Getting Started Article](/articles/en/getting_started) for a guide on installing eiipm and ewwii.

## Adding eiipm to path

**This is a very important step** which people are likely to miss. By default, eiipm installs binaries to `~/.eiipm/bin` directory. But your shell doesn't know about it yet.

So, when you run something like `bin-you-installed` after installing a binary from eiipm, your shell will go like "Oh, let me check in all the known areas. Nope, `bin-you-installed` is not installed..."

So, you should add `export PATH="$HOME/.eiipm/bin:$PATH"` to your shell's configuration file.

**Here is an example on how to do it:**

```bash
# Replace ~/.zshrc with your shell's configuration file.
# For example, if you use bash, then it would be ~/.bashrc
echo 'export PATH="$HOME/.eiipm/bin:$PATH"' >> ~/.zshrc
```

I use zsh, so I added the line `export PATH="$HOME/.eiipm/bin:$PATH"` in `~/.zshrc` but if you use something else, you should replace the `~/.zshrc` with your own shell's confiuration file.

For example, if you use bash, add that line in `~/.bashrc`.

> **NOTE:** If you dont want to use echo to add it, then you can manually edit your configuration file and add the line `export PATH="$HOME/.eiipm/bin:$PATH"` in there.
