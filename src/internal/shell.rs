use anyhow::Context;
use thiserror::Error;

/// An abstraction around a shell that can run commands on the host system.
pub struct Shell {}

impl Shell {
    /// Creates a new `Shell`.
    pub fn new() -> Self {
        Self {}
    }

    /// Runs the given command with the given args (if any) and returns the result.
    pub fn run_command(&self, cmd: String, args: Vec<String>) -> Result<String, ShellError> {
        let res = std::process::Command::new(&cmd).args(&args).output();
        if let Err(err) = res {
            return Err(ShellError::ShellStartFailure {
                command: cmd,
                args: args.join(" "),
                error: err.to_string(),
            }
            .into());
        }
        let output = res.unwrap();

        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            Err(ShellError::HostProcessExecutionFailure {
                command: cmd,
                args: args.join(" "),
                status: output.status.to_string(),
                stdout: String::from_utf8(output.stdout).unwrap(),
                stderr: String::from_utf8(output.stderr).unwrap(),
            })
        }
    }
}

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("failed to execute command (ran: '{command} {args}', got status: {status})")]
    HostProcessExecutionFailure {
        command: String,
        args: String,
        status: String,
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
        $shell.run_command($cmd.to_string(), args![$($arg),*])
    };
}
