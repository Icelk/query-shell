use std::str::FromStr;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("The platform is not supported")]
    UnsupportedPlatform,
    #[error("Current process does not have a parent")]
    NoParent,
    #[error("Unknown shell")]
    Unknown,
    #[error("Unavailable with some su implementations")]
    InSu,
}

#[must_use]
pub fn get_shell_name() -> Result<String, Error> {
    let sys = System::new_all();
    let process = sys
        .get_process(get_current_pid().map_err(|_| Error::UnsupportedPlatform)?)
        .expect("Process with current pid does not exist");
    let parent = sys
        .get_process(process.parent().ok_or(Error::NoParent)?)
        .expect("Process with parent pid does not exist");
    let shell = parent.name().trim().to_lowercase();
    let shell = shell.strip_suffix(".exe").unwrap_or(&shell); // windows bad
    let shell = shell.strip_prefix("-").unwrap_or(&shell); // login shells
    Ok(shell.to_owned())
}
#[must_use]
pub fn get_shell() -> Result<Shell, Error> {
    Shell::get()
}

#[derive(Debug)]
pub enum Shell {
    Bash,
    Elvish,
    Fish,
    Ion,
    Nushell,
    Powershell,
    Xonsh,
    Zsh,
}

impl Shell {
    #[must_use]
    pub fn get() -> Result<Self, Error> {
        match get_shell_name()?.as_str() {
            "su" => Err(Error::InSu),
            shell if shell.starts_with("python") => Ok(Self::Xonsh),
            shell => Self::from_str(shell),
        }
    }
}

impl FromStr for Shell {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bash" => Ok(Shell::Bash),
            "elvish" => Ok(Shell::Elvish),
            "fish" => Ok(Shell::Fish),
            "ion" => Ok(Shell::Ion),
            "nu" | "nushell" => Ok(Shell::Nushell),
            "pwsh" | "powershell" => Ok(Shell::Powershell),
            "xonsh" | "xon.sh" => Ok(Shell::Xonsh),
            "zsh" => Ok(Shell::Zsh),
            _ => Err(Error::Unknown),
        }
    }
}
