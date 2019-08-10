use crate::config::Config;
use crate::error::MargeError;
use crate::git;
use rand::seq::SliceRandom;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;

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
    api_key: &str,
    project_name: &str,
) -> Result<Vec<Project>, MargeError> {
    let request = server_url.to_owned() + "api/v4/projects";

    let res = reqwest::Client::new()
        .get(&request)
        .headers(construct_headers(api_key))
        .query(&[("search", project_name)])
        .send()?
        .json::<Vec<Project>>()?;

    Ok(res)
}

pub fn create_merge_request(config: Config) -> Result<(), MargeError> {
    let params = json!({
        "id": config.project_id,
        "source_branch": git::active_branch()?,
        "target_branch": config.default_branch,
        "assignee_id": chose_assignee(),
        "title": "This is a test",
        "description": "a description for this very useful mr",
        "remove_source_branch": true,
        "squash": true,
    });

    let url = Url::parse(&config.server_uri).unwrap();
    let mr_path = String::from("/api/v4/projects/") + &config.project_id.to_string() + "/merge_requests";
    url.join(&mr_path).unwrap();

    dbg!(&mr_path);

    reqwest::Client::new()
        .post(url.as_str())
        .headers(construct_headers(&config.api_key))
        .json(&params)
        .send()?;

    Ok(())
}

fn construct_headers(auth_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let value = HeaderValue::from_str(&auth_key).expect("wrong auth key");
    headers.insert("Private-Token", value);

    headers
}

#[allow(dead_code)]
fn chose_assignee() -> i32 {
    // Adam, Manfred, Anzor
    let commitor_ids = [81, 69, 60];
    *commitor_ids.choose(&mut rand::thread_rng()).unwrap()
}
