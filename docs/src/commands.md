# Commands

Eiipm offers many commands which users can use to manage plugins.

```bash
# Init plugin manager
eiipm init

# Add plugins
eiipm add user/repo
eiipm add user/repo --ref v1.2.0  # pin to a tag
eiipm add user/repo --prebuilt    # prefer prebuilt binary if available
eiipm add user/repo --build "cargo build --release" --artifact "target/release/libmy-plugin.so"

# Install plugins
eiipm install

# Update Plugins
eiipm update           # update all plugins
eiipm update user/repo # update only this plugin

# Remove plugins
eiipm remove user/repo

# List plugins
eiipm list

# Cleaning
eiipm clean        # remove untracked artifacts from 'plugins/'
eiipm cache-clean  # wipe the global source cache (~/.cache/eiipm/)
```
