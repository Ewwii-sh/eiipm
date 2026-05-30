# Authoring Pluign

If you are writing a plugin for ewwii and want it to be compatible with eiipm, then add a `plugin.toml` file at the root of your repository:

```toml
[plugin]
build = "cargo build --release"
artifact = "target/release/libmyplugin.so"

# optional: provide a prebuilt binary for users who don't want to build
[plugin.prebuilt]
url = "https://github.com/user/repo/releases/download/{version}/libmyplugin-{arch}-{os}.so"
```
