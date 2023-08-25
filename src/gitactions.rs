use std::{
    path::Path,
    process::{Command, Output},
};

use crate::errors::{DotManResult, Error, GitError};

pub struct GitWrapper {
    url: String,
    path: String,
}

impl GitWrapper {
    pub fn open(url: &str, path: &str) -> DotManResult<GitWrapper> {
        let wrapper = Self {
            url: url.to_owned(),
            path: path.to_owned(),
        };

        // HACK: If this fails, then it isn't a git repo
        wrapper.current_branch_name()?;

        Ok(wrapper)
    }

    pub fn clone(url: &str, path: &str) -> DotManResult<Self> {
        println!("{:#?}", path);
        let output = Command::new("git").args(["clone", url, path]).output()?;

        let wrapper = Self {
            url: url.to_owned(),
            path: path.to_owned(),
        };

        if !output.status.success() {
            return Err(wrapper.handle_error(output));
        }

        Ok(wrapper)
    }

    pub fn current_branch_name(&self) -> DotManResult<String> {
        let output = Command::new("git")
            .current_dir(Path::new(&self.path))
            .args(["branch", "--show-current"])
            .output()?;

        if !output.status.success() {
            return Err(self.handle_error(output));
        }

        Ok(String::from_utf8(output.stdout)?.trim().to_owned())
    }

    pub fn checkout_branch(&self, name: &str) -> DotManResult<()> {
        let output = Command::new("git")
            .current_dir(Path::new(&self.path))
            .args(["checkout", name])
            .output()?;

        if !output.status.success() {
            return Err(self.handle_error(output));
        }

        Ok(())
    }

    pub fn get_remote_name_from_url(&self, url: &str) -> DotManResult<String> {
        let output = Command::new("git")
            .current_dir(Path::new(&self.path))
            .args(["remote", "-v"])
            .output()?;

        if !output.status.success() {
            return Err(self.handle_error(output));
        }

        let output = String::from_utf8(output.stdout)?;

        for line in output.lines() {
            if line.contains(url) {
                return Ok(line
                    .split('\t')
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .to_string());
            }
        }

        Err(Error::RemoteNotFound(self.url.to_owned()))
    }

    pub fn fetch(&self, remote: &str) -> DotManResult<()> {
        let output = Command::new("git")
            .current_dir(Path::new(&self.path))
            .args(["fetch", remote, "master"])
            .output()?;

        if !output.status.success() {
            return Err(self.handle_error(output));
        }

        Ok(())
    }

    pub fn rebase(&self) -> DotManResult<()> {
        let output = Command::new("git")
            .current_dir(Path::new(&self.path))
            .args(["rebase", "master"])
            .output()?;

        if !output.status.success() {
            return Err(self.handle_error(output));
        }

        Ok(())
    }

    pub fn pull(&self, remote: &str) -> DotManResult<()> {
        let output = Command::new("git")
            .current_dir(Path::new(&self.path))
            .args(["pull", remote, "master"])
            .output()?;

        if !output.status.success() {
            return Err(self.handle_error(output));
        }

        Ok(())
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    fn handle_error(&self, output: Output) -> Error {
        let output = match String::from_utf8(output.stderr) {
            Ok(o) => o,
            Err(e) => return Error::from(e),
        };

        if output.contains("not a git repository") {
            Error::Git(GitError::NotARepository(format!(
                "{} is not a git repo",
                self.path
            )))
        } else {
            Error::Git(GitError::Unknown(format!("{}: unknown error", self.path)))
        }
    }
}
