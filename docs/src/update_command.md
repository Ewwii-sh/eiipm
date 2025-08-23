# Update

### Command

Update a package or all packages

```bash
eiipm update [PACKAGE]
# or shorter:
eiipm up [PACKAGE]
```

### Description

- If PACKAGE is given, then it updates that package.
- If no package is given, then it updates all installed packages.

### Options

**--debug**: Show debug logs.

### Examples

```bash
eiipm up # update all
eiipm up staticscript  # update just one package
```
