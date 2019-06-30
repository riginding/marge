use git2::Repository;
use crate::error::MargeError;
use std::path::PathBuf;

fn current_repo() -> Result<Repository, MargeError> {
    let path = std::env::current_dir()?;
    let repository = git2::Repository::discover(path)?;

    Ok(repository)
}

pub fn active_branch() -> Result<String, MargeError> {
    let name =current_repo()?
        .head()
        .unwrap()
        .shorthand()
        .unwrap()
        .to_owned();

    Ok(name)
}

pub fn git_path() -> Result<PathBuf, MargeError> {
    let path = current_repo()?.path().to_owned();
    Ok(path)
}
