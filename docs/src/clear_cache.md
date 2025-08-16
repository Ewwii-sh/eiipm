# Clear Cache

### Command

Clear package cache

```bash
eiipm clear-cache [PACKAGE]
# or shorter:
eiipm cc [PACKAGE]
```

### Description

- If PACKAGE is given, then it clears the cache of that package.
- If no package is given, then it clears cache of all installed packages.

### Options

**--debug**: Show debug logs.

### Examples

```bash
eiipm cc # clear all
eiipm clear-cache statictranspl  # clear just one package cache
```
