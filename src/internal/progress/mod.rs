use colored::Colorize;
use spinners::{Spinner, Spinners};
use std::time::Instant;

pub struct Progress;

impl Progress {
    /// Constructs a new progress spinner with the given prompt.
    ///
    /// If the current logger is set to `off`, the spinner will not be displayed.
    pub fn new(prompt: &str) -> Box<dyn ProgressInterface> {
        if log::max_level() == log::LevelFilter::Off {
            Box::new(SilentProgressImpl)
        } else {
            Box::new(VisibleProgressImpl::new(prompt))
        }
    }
}

pub trait ProgressInterface {
    /// Stops the spinner and prints a success message.
    fn success(&mut self);

    /// Stops the spinner and prints a failure message.
    fn fail(&mut self);
}

struct VisibleProgressImpl {
    prompt: String,
    start_time: Instant,
    spinner: Spinner,
}

impl VisibleProgressImpl {
    fn new(prompt: &str) -> Self {
        let now = Instant::now();

        let initial_prompt = format!("{}...", prompt).bold().to_string();
        Self {
            prompt: prompt.into(),
            start_time: now,
            spinner: Spinner::new(Spinners::Dots, initial_prompt),
        }
    }
}

impl ProgressInterface for VisibleProgressImpl {
    fn success(&mut self) {
        self.spinner.stop_with_message(format!(
            "{} {} (took {:.2}ms)",
            "✔".green(),
            self.prompt,
            self.start_time.elapsed().as_micros() / 1000
        ));
    }

    fn fail(&mut self) {
        let prompt = self.prompt.bold();

        self.spinner.stop_with_message(format!(
            "{} {} (took {:.2}ms)",
            "✘".red(),
            prompt,
            self.start_time.elapsed().as_micros() / 1000
        ));
    }
}

struct SilentProgressImpl;

impl ProgressInterface for SilentProgressImpl {
    fn success(&mut self) {}

    fn fail(&mut self) {}
}

/// A macro that wraps a function in a progress spinner.
///
/// The spinner will complete successfully if the function returns `Ok(())` or
/// fail if the function returns `Err(_)`. After the function returns, the
/// spinner will be stopped and a message will be printed to the console, and
/// the returned value from the function will be returned from the macro.
///
/// # Examples
/// ```no_run
/// let num = progress!("get number", get_number());
///
/// fn get_number() -> Result<i32, ()> {
///   Ok(42)
/// }
///
/// assert_eq!(num, Ok(42));
/// ```
#[macro_export]
macro_rules! progress {
    ($prompt:expr, $expr:expr) => {{
        let mut progress = $crate::internal::progress::Progress::new($prompt);
        let result = $expr;
        match &result {
            Ok(_) => progress.success(),
            Err(_) => progress.fail(),
        }
        result
    }};
}
