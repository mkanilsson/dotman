use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::errors::DotManResult;

pub fn run_postinstall(dir: &str) -> DotManResult<()> {
    run_script(dir, ".dotman-postinstall")
}

pub fn run_postupdate(dir: &str) -> DotManResult<()> {
    run_script(dir, ".dotman-postupdate")
}

fn run_script(dir: &str, script: &str) -> DotManResult<()> {
    let path = Path::new(dir).join(script);
    if !path.exists() {
        return Ok(());
    }

    Command::new("/usr/bin/bash")
        .args(["-xe", path.to_str().unwrap()])
        .current_dir(dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    // TODO: Report back if script failed
    Ok(())
}
