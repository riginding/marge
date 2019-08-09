use crate::error::MargeError;
use crate::git::git_path;
use crate::gitlab::search_for_project;
use crate::gitlab::Project;
use crate::Result;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Confirmation, Select};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server_uri: String,
    pub api_key: String,
    pub project_name: String,
    pub project_id: i32,
    pub default_branch: String,
}

impl Config {
    pub fn init() -> Result<()> {
        let path = get_config_path()?;
        if path.exists()
            && !Confirmation::with_theme(&ColorfulTheme::default())
                .with_text(".marge config file found, do you want to overwrite it?")
                .interact()?
        {
            return Ok(());
        }

        Ok(write_configuration()?)
    }

    pub fn new() -> Result<Config> {
        let server_uri = query_server()?;
        let api_key = query_api_key()?;
        let project_name = query_project_name()?;
        let pj = search_for_project(&server_uri, &api_key, &project_name)?;
        let project_id = pick_a_project(&pj)?;
        let default_branch = query_default_branch()?;

        Ok(Config {
            server_uri,
            api_key,
            project_name,
            project_id,
            default_branch,
        })
    }
}

fn get_config_path() -> Result<PathBuf> {
    let path = git_path()?
        .parent()
        .ok_or(MargeError::PathNoParentError)?
        .join(".marge");

    Ok(path)
}

fn write_configuration() -> Result<()> {
    let path = get_config_path()?;
    let mut file = File::create(&path)?;
    let configuration = Config::new()?;
    let config_string = serde_yaml::to_string(&configuration).unwrap();
    file.write_all(config_string.as_bytes())?;

    Ok(())
}

fn pick_a_project(projects: &[Project]) -> Result<i32> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your project")
        .default(0)
        .items(projects)
        .interact()?;

    Ok(projects[selection].id)
}

fn query_server() -> Result<String> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter gitlab URL")
        .default(String::from("https://git.sclable.com/"))
        .interact()
        .map_err(|_| MargeError::IOError)
}

fn query_api_key() -> Result<String> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter API key")
        .interact()
        .map_err(|_| MargeError::IOError)
}

fn query_project_name() -> Result<String> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("enter project name to search")
        .interact()
        .map_err(|_| MargeError::IOError)
}

fn query_default_branch() -> Result<String> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("enter projects default branch")
        .default(String::from("sandbox"))
        .interact()
        .map_err(|_| MargeError::IOError)
}
