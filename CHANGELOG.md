## 0.1.5 (2024-07-15)

- chore: Updated Rust to the latest stable version.
- chore: Updated all dependencies to their latest versions, including major incompatible ones.
- fix: Resolved doc test failures for `progress!` and `exec_on!` macros caused by missing imports after dependency updates.
- fix: Addressed warnings related to redundant `.deref()` calls that appeared after dependency updates.
- fix: Removed unused `Deref` imports after fixing the aforementioned warnings.

## 0.1.4 (2023-09-29)

- fix: ignore more special directories (such as `flutter_gen`) when running commands that run for every Dart/Flutter project (like `ford` and `fua`)

## 0.1.3 (2023-04-13)

- feat: add `fua` command
- feat: add `--offline` flag to `upgrade` command
- fix: ignore certain directories when running commands that run for every Dart/Flutter project (like `ford` and `fua`)

## 0.1.2 (2023-03-17)

- feat: add `ford` command
- refactor: rename `update` command to `upgrade`
- feat: add `--quiet` flag to all commands (completely disables output)
- chore: improve verbose logging

## 0.1.1 (2023-03-16)

- docs: documentation upgrades and fixes

## 0.1.0 (2023-03-16)

- feat: initial project setup
- feat: add `gho` command
- feat: add `update` command
