# Install

### Command

Install a package

```bash
eiipm install <PACKAGE>
# or shorter:
eiipm i <PACKAGE>
```

### Description

Downloads, builds (if needed), and installs the given package.

### Options

**--debug**: Show debug logs.

### Examples

```bash
eiipm install staticscript
eiipm i ewwii
```

### Security Notice

<div class="warning">
    Eiipm installs packages listed in the <a href="https://github.com/Ewwii-sh/eii-manifests">eii-manifests</a> repository. Since third-party packages may contain vulnerabilities, always ensure you trust the author, even if a package is officially approved and included in <a href="https://github.com/Ewwii-sh/eii-manifests">eii-manifests</a> repository.  
    <br><br>
    Malicious or poorly maintained packages can compromise system security, cause data loss, or introduce unexpected behavior.  
    Before installing, review the package source, check for recent updates, and verify community feedback whenever possible.  
    Exercise caution and follow best practices to minimize potential risks when using third-party packages.
</div>
