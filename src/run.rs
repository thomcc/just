use crate::common::*;

/// Invoke `just` with the provided arguments.
pub fn run<I>(args: I) -> Result<(), RunError>
where
  I: IntoIterator,
  I::Item: Into<std::ffi::OsString> + Clone,
{
  #[cfg(windows)]
  ansi_term::enable_ansi_support().ok();

  env_logger::Builder::from_env(
    env_logger::Env::new()
      .filter("JUST_LOG")
      .write_style("JUST_LOG_STYLE"),
  )
  .init();

  let app = Config::app();

  info!("Parsing command line arguments…");
  let matches = app.get_matches_from(args);

  let config = Config::from_matches(&matches)
    .eprint(Color::auto())
    .map_err(RunError)?;

  config.run_subcommand().map_err(RunError)
}

/// A mostly-opaque error indicating that the requested `just` command failed.
#[derive(Debug)]
pub struct RunError(i32);

impl RunError {
  /// Returns the status code of this error.
  pub fn code(&self) -> i32 {
    self.0
  }
}

impl std::fmt::Display for RunError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "just::run failed with code {}", self.code())
  }
}

impl std::error::Error for RunError {}
