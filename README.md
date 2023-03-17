# Suitcase 💼

A set of personal CLI tools to automate common tasks in software development (including Rust, Dart, and Flutter), written in Rust.

> ⚠️ This is a **work in progress** Rust port of [jeroen-meijer/suitcase][dart-repo]. Expect breaking changes.

## Installation

> Note: may be inaccurate and change over time.

### Cargo

```bash
$ cargo install suitcase
```

### Manual

```bash
$ git clone https://github.com/jeroen-meijer/suitcase-rs.git
$ cd suitcase-rs
$ cargo install --path .
```

## Commands

This is a list of all commands that are currently available or planned to be implemented. All commands can be invoked by running `suitcase <COMMAND>` or by running the command directly (e.g. `gho`), except for `suitcase upgrade`.

| Command            | Full Name              | Description                                                                                                          | Example                 | Status         |
| ------------------ | ---------------------- | -------------------------------------------------------------------------------------------------------------------- | ----------------------- | -------------- |
| `suitcase`         | Suitcase               | The main command. Use this to spawn any subcommands.                                                                 | `suitcase <SUBCOMMAND>` | ✅ Implemented |
| `suitcase help`    | Help                   | Show a list of all commands or detailed help for a specific command.                                                 | `suitcase help`         | ✅ Implemented |
| `suitcase upgrade` | upgrade                | upgrade the Suitcase CLI to the latest version (either from a local path or crates.io).                              | `suitcase upgrade`      | ✅ Implemented |
| `gho <PATH>`       | GitHub Open            | Open the current Git repository in the default browser (supports GitHub, GitLab, and any other Git hosting service). | `gho ./`                | ✅ Implemented |
| `ford <COMMAND>`   | For Every Dart Project | Run a command for every Dart project in the current directory and any subdirectories (recursively).                  | `ford pub get`          | ✅ Implemented |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

[repo]: https://github.com/jeroen-meijer/suitcase-rs
[dart-repo]: https://github.com/jeroen-meijer/suitcase
