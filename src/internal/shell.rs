use log::debug;
use std::process::ExitStatus;
use thiserror::Error;

/// An abstraction around a shell that can run commands on the host system.
pub struct Shell {}

impl Shell {
    /// Creates a new `Shell`.
    pub fn new() -> Self {
        Self {}
    }

    /// Indents the given string by the given number of spaces.
    fn indent(spaces: usize, string: &str) -> String {
        let indent = " ".repeat(spaces);
        string
            .lines()
            .map(|line| format!("{}{}", indent, line))
            .collect::<Vec<_>>()
            .join("\r")
    }

    /// Run the given command with the given args (if any) and returns the result.
    pub fn run_command(
        &self,
        cmd: String,
        args: Vec<String>,
        // An optional callback that will receive lines of stdout as they are produced.
    ) -> Result<ShellOutput, ShellError> {
        debug!("running command: {} {}", cmd, args.join(" "));

        let output = std::process::Command::new(&cmd)
            .args(&args)
            .output()
            .map_err(|err| ShellError::ShellStartFailure {
                command: cmd.clone(),
                args: args.join(" "),
                error: err.to_string(),
            })?;

        let output = ShellOutput {
            status: output.status,
            stdout: String::from_utf8(output.stdout).unwrap(),
            stderr: String::from_utf8(output.stderr).unwrap(),
        };

        debug!("command status: {}", output.status);
        debug!(
            "command output:\n  stdout:\n{}\n\n  stderr:\n{}",
            if output.stdout.is_empty() {
                Shell::indent(4, "<NO STDOUT OUTPUT>").into()
            } else {
                format!("\n{}", Shell::indent(4, &output.stdout))
            },
            if output.stderr.is_empty() {
                Shell::indent(4, "<NO STDERR OUTPUT>").into()
            } else {
                format!("\n{}", Shell::indent(4, &output.stderr))
            }
        );

        if output.status.success() {
            Ok(output)
        } else {
            Err(ShellError::HostProcessExecutionFailure {
                command: cmd,
                args: args.join(" "),
                status: output.status,
                stdout: output.stdout,
                stderr: output.stderr,
            })
        }
    }
}

pub struct ShellOutput {
    pub status: std::process::ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Error, Debug, Clone)]
pub enum ShellError {
    #[error("failed to execute command (ran: '{command} {args}', got status: {status}, stdout: '{stdout}', stderr: '{stderr}')")]
    HostProcessExecutionFailure {
        command: String,
        args: String,
        status: ExitStatus,
        stdout: String,
        stderr: String,
    },

    #[error("failed to start shell: {error} (ran: '{command} {args}')")]
    ShellStartFailure {
        command: String,
        args: String,
        error: String,
    },
}

/// A macro that attempts to convert all given arguments into a `String`
/// for use as a command line argument by calling `to_string()` on each one.
///
/// # Examples
/// ```
/// use suitcase::args;
///
/// let args: Vec<String> = args!["foo", "bar", 42];
/// assert_eq!(args, vec!["foo".to_string(), "bar".to_string(), "42".to_string()]);
/// ```
#[macro_export]
macro_rules! args {
    ($($arg:expr),*) => {
        vec![$($arg.to_string()),*]
    };
}

/// A macro that runs the given command with the given args (if any)
/// on the given shell and returns the result.
///
/// # Examples
/// ```no_run
/// let shell = suitcase::internal::shell::Shell::new();
///
/// let result = exec_on!(shell, "echo", "hello world").unwrap();
/// assert_eq!(result, "hello world\n");
/// ```
#[macro_export]
macro_rules! exec_on {
    ($shell:expr, $cmd:expr) => {
        $shell.run_command($cmd.to_string(), vec![])
    };
    ($shell:expr, $cmd:expr, $($arg:expr),*) => {
        $shell.run_command($cmd.to_string(), $crate::args![$($arg),*])
    };
}
