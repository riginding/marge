use crate::error::MargeError;
use crate::git;
use rand::seq::SliceRandom;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i32,
    name_with_namespace: String,
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name_with_namespace)
    }
}

pub fn search_for_project(
    server_url: &str,
    _api_key: &str,
    project_name: &str,
) -> Result<Vec<Project>, MargeError> {
    let request = server_url.to_owned() + "api/v4/projects";

    let res = reqwest::Client::new()
        .get(&request)
        .headers(construct_headers())
        .query(&[("search", project_name)])
        .send()?
        .json::<Vec<Project>>()?;

    Ok(res)
}

#[allow(dead_code)]
pub fn create_merge_request() -> Result<(), MargeError> {
    let params = json!({
        "id": "506", // dreamfactory hardcoded
        "source_branch": git::active_branch()?,
        "target_branch": "sandbox",
        "assignee_id": chose_assignee(),
        "title": "This is a test",
        "description": "a description for this very useful mr",
        "remove_source_branch": true,
        "squash": true,
    });

    reqwest::Client::new()
        .post("https://git.sclable.com/api/v4/projects/506/merge_requests")
        .headers(construct_headers())
        .json(&params)
        .send()?;

    Ok(())
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Private-Token",
        HeaderValue::from_static("DuBDeordPxNvJFEruwdT"),
    );

    headers
}

#[allow(dead_code)]
fn chose_assignee() -> i32 {
    // Adam, Manfred, Anzor
    let commitor_ids = [81, 69, 60];
    *commitor_ids.choose(&mut rand::thread_rng()).unwrap()
}
