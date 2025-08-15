# eiipm

**Eiipm** pronounced as **ee-pee-em** is the package manager of Ewwii (Elkowar's Wacky Widgets Improved Interface).

## Common commands

The most common commands in eiipm are the `install`, `uninstall` and `update` commands. Just like their name, they are used to install, uninstall and update packages that you have installed.

**Usage example:**

```bash
# installs the statictranspl binary
eiipm install statictranspl

# uninstalls the statictranspl binary
eiipm uninstall statictranspl

# updates the statictranspl binary to the latest version
eiipm update statictranspl
```

## Adding to path

If you install a binary from **eiipm**, it may not work if you type the name of the binary in the terminal directly. You would need to add eiipm to your path.

To add eiipm to your path, add `export PATH="$HOME/.eiipm/bin:$PATH"` to your shell's configuration file.

**Example:**

```bash
# Replace ~/.zshrc with your shell's configuration file.
# For example, if you use bash, then it would be ~/.bashrc
echo 'export PATH="$HOME/.eiipm/bin:$PATH"' >> ~/.zshrc
```

If you dont want to use echo to add it, then you can manually edit your configuration file and adding the line `export PATH="$HOME/.eiipm/bin:$PATH"` in there.

## Uploading a custom plugin

If you made a custom plugin and want to register it to ewwii's package manifest, then you should checkout the [Ewwii-sh/eii-manifests](https://github.com/Ewwii-sh/eii-manifests) repository for more info.
