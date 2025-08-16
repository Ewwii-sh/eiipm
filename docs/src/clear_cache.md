# Clear Cache

### Command

Clear package cache with confirmation.

```bash
eiipm clear-cache [PACKAGE]
# or shorter:
eiipm cc [PACKAGE]
```

### Description

- If PACKAGE is given, then it clears the cache of that package with confirmation.
- If no package is given, then it clears cache of all installed packages with confirmation.

### Options

**--force**: Bypass confirmation.
**--debug**: Show debug logs.

### Examples

```bash
eiipm cc # clear all
eiipm clear-cache statictranspl  # clear just one package cache

eiipm cc --force # bypasses confirmation
```
