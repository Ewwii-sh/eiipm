# Examples

Installing [yucky-ewwii](https://github.com/ewwii-sh/yucky-ewwii):

```bash
# initialize
eiipm init

# add yucky-ewwii
eiipm add yucky-ewwii --prebuilt

# install
eiipm install
```

**Explanation:**

- `eiipm init` will initialize the necessary directories and files. 
- `eiipm add yucky-ewwii --prebuilt` will add yucky-ewwii to your configuration (by adding it inside `plugins.toml`) and tries to use the prebuilt artifact to avoid compilation which can be slow. 
- `eiipm install` will install all the plugins that you added, that being the `yucky-ewwii` plugin.
