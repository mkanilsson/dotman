use std::{fs, path::Path, process::Command};

use crate::errors::DotManResult;

pub fn run_postinstall(dir: &str) -> DotManResult<()> {
    let path = Path::new(dir).join(".dotman-postinstall");
    if !path.exists() {
        return Ok(());
    }

    Command::new("/usr/bin/bash")
        .args([path])
        .current_dir(dir)
        .output()?;

    // TODO: Report back if script failed
    return Ok(());
}

pub fn run_postupdate(dir: &str) -> DotManResult<()> {
    let path = Path::new(dir).join(".dotman-postupdate");
    if !path.exists() {
        return Ok(());
    }

    Command::new("/usr/bin/bash")
        .args([path])
        .current_dir(dir)
        .output()?;

    // TODO: Report back if script failed
    return Ok(());
}
