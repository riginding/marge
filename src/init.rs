use crate::error::MargeError;
use crate::git::git_path;
use crate::gitlab::search_for_project;
use crate::gitlab::Project;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Confirmation, Select};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub server_uri: String,
    pub api_key: String,
    pub project_name: String,
    pub project_id: i32,
    pub default_branch: String,
}

pub fn find_or_create() -> Result<(), MargeError> {
    let path = git_path()?;
    let path = path
        .parent()
        .ok_or(MargeError::PathNoParentError)?
        .join(".marge");

    let file = OpenOptions::new().read(true).write(true).open(&path);

    match file {
        Ok(file) => {
            if Confirmation::with_theme(&ColorfulTheme::default())
                .with_text(".marge config file found, do you want to overwrite it?")
                .interact()?
            {
                create_configuration(file)?;
            }
        }
        Err(_) => {
            let file = File::create(&path).unwrap();
            create_configuration(file)?;
        }
    }

    Ok(())
}

fn create_configuration(mut file: File) -> Result<(), MargeError> {
    let server_uri = query_server()?;
    let api_key = query_api_key()?;
    let project_name = query_project_name()?;
    let pj = search_for_project(&server_uri, &api_key, &project_name)?;
    let project_id = pick_your_project(&pj)?;
    let default_branch = query_default_branch()?;

    let configuration = Configuration {
        server_uri,
        api_key,
        project_name,
        project_id,
        default_branch,
    };

    let config_string = serde_yaml::to_string(&configuration).unwrap();
    file.write_all(config_string.as_bytes())?;

    Ok(())
}

fn pick_your_project(projects: &[Project]) -> Result<i32, MargeError> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your project")
        .default(0)
        .items(projects)
        .interact()?;

    Ok(projects[selection].id)
}

fn query_server() -> Result<String, MargeError> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter gitlab URL")
        .default(String::from("https://git.sclable.com/"))
        .interact()
        .map_err(|_| MargeError::IOError)
}

fn query_api_key() -> Result<String, MargeError> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter API key")
        .interact()
        .map_err(|_| MargeError::IOError)
}

fn query_project_name() -> Result<String, MargeError> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("enter project name to search")
        .interact()
        .map_err(|_| MargeError::IOError)
}

fn query_default_branch() -> Result<String, MargeError> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("enter projects default branch")
        .default(String::from("sandbox"))
        .interact()
        .map_err(|_| MargeError::IOError)
}
