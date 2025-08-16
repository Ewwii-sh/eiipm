# Check Updates

### Command

Check for updates in a package or all packages

```bash
eiipm check-update [PACKAGE]
# or shorter:
eiipm cu [PACKAGE]
```

### Description

- If PACKAGE is given, then it checks for updates in that package.
- If no package is given, then it checks for updates in all installed packages.

### Options

**--debug**: Show debug logs.

### Examples

```bash
eiipm cu # check all
eiipm check-updates statictranspl  # check just one package
```
