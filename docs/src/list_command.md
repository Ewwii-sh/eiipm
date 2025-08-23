# List

### Command

List installed packages

```bash
eiipm list [OPTIONS]
# or shorter:
eiipm l [OPTIONS]
```

### Description

Shows installed packages. Can display just names, detailed info, total count, or a specific package.

### Options

- **-v**, **--verbose**: Show detailed info for each package.
- **-t**, **--total-count**: Show only the total number of installed packages.
- **-q**, **--query** &lt;NAME&gt;: Show info for a single package (works with --verbose as well).
- **--debug**: Show debug logs.

### Examples

```bash
eiipm l
eiipm l --verbose
eiipm l --total-count
eiipm l --query staticscript
eiipm l -q staticscript -v
```
