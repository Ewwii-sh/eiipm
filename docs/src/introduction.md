# Introduction

Eiipm is a fast and eligant package manager made in rust for [Ewwii](https://github.com/Ewwii-sh/ewwii). Eiipm uses the package metadata from the [Ewwii-sh/eii-manifests](https://github.com/Ewwii-sh/eii-manifests) repository where the manifests of packages are stored.

## Installation

You can install **eiipm** using the same [methods we discussed](https://ewwii-sh.github.io/ewwii/installation.html) of in Ewwii:

#### 1. From installer (Linux Only)

```bash
curl -sSL https://raw.githubusercontent.com/Ewwii-sh/eiipm/main/install.sh -o install.sh
less install.sh # inspect installer code (optional)
sh install.sh # run the installer
```

After, after the installation, verify it works:

```bash
eiipm --version
```

> The installer installs the binary to `/usr/local/bin`
>
> If eiipm doesn't work after the installation, make sure that `/usr/local/bin` is in path.

#### 2. From GitHub releases (Linux Only)

If you are on Linux, then you can install the pre-built binary from [eiipm GitHub releases](https://github.com/Ewwii-sh/eiipm/releases/latest).

- Go to the latest GitHub release. [Click Me!](https://github.com/Ewwii-sh/eiipm/releases/latest)
- Install the binary (that is named `eiipm`) from assets section.
- Voila! You now have eiipm installed!

#### 3. From source

```bash
git clone https://github.com/Ewwii-sh/eiipm
cd eiipm
cargo build --release
```

This will generate the `eiipm` binary in `target/release`.

#### 4. Using Cargo

```bash
cargo install --git https://github.com/Ewwii-sh/eiipm
```

After installation, verify it works:

```bash
eiipm --version
```

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
