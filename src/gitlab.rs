use rand::seq::SliceRandom;
use reqwest::header::{HeaderMap, HeaderValue};
use std::path::{Path, PathBuf};
use crate::error::MargeError;
use git2::Repository;

pub fn create_merge_request() -> Result<(), MargeError> {
    let params = json!({
        "id": "506", // dreamfactory hardcoded
        "source_branch": active_branch(),
        "target_branch": "sandbox",
        "assignee_id": chose_assignee(),
        "title": "This is a test",
        "description": "a description for this very useful mr",
        "remove_source_branch": true,
        "squash": true,
    });

    let client = reqwest::Client::new();
    client
        .post("https://git.sclable.com/api/v4/projects/506/merge_requests")
        .headers(construct_headers())
        .json(&params)
        .send()?;

    println!("{:?}", params);

    Ok(())
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Private-Token",
        HeaderValue::from_static("wJ7Xg4gWAB4zenVMvysc"),
    );

    headers
}

fn chose_assignee() -> i32 {
    // Adam, Manfred, Anzor
    let commitor_ids = [81, 69, 60];
    *commitor_ids.choose(&mut rand::thread_rng()).unwrap()
}

fn current_repo() -> Result<Repository, MargeError> {
    let path = std::env::current_dir()?;
    let repository = git2::Repository::discover(path)?;

    Ok(repository)
}

fn active_branch() -> Result<String, MargeError> {
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
