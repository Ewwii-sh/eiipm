# Changelog

All notable changes to `eiipm` are documented here.

This changelog follows the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format,
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.4.0] - [UNRELEASED]

### Added

- Added commit hash based install/update for security reasons.
- `install.sh` file for easy install in Linux.

## [0.3.0] - 2025-08-22

### Added

- Support for unix glob patterns in files key (toml).
- **Search** command with `--log-metadata` (or `-l`) flag.

### Changed

- Vendored libgit2 to be compiled along with eiipm.

## [0.2.0] - 2025-08-18

### Added

- `src`, `dest` based file copying. See [eii-manifests](https://github.com/Ewwii-sh/eii-manifests?tab=readme-ov-file#step-4-advanced-files-options).
- A check to see if package needs update before updating.
- **list-cache** (or **lc**) command for listing all cache.
- **purge-cache** (or **pc**) command to remove broken/orphaned cache.

### Removed

- Updation of themes.

## [0.1.0] - 2025-08-17

### Added

- **install** command
- **uninstall** command
- **update** command
- **list** command
- **clear-cache** command
- **check-update** command
- **[git2](https://docs.rs/git2/latest/git2/)** based version control
- **Eiipm** not in path warning
