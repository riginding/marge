use crate::error::MargeError;
use crate::git::git_path;
use crate::gitlab::search_for_project;
use crate::gitlab::Project;
use crate::Result;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Confirmation, Select};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Buddy {
    pub name: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub server_uri: String,
    pub api_key: String,
    pub project_name: String,
    pub project_id: i32,
    pub default_branch: String,
    pub buddies: Vec<Buddy>,
}

impl Config {
    pub fn init() -> Result<Self> {
        if Config::exists()
            && !Confirmation::with_theme(&ColorfulTheme::default())
                .with_text(".marge config file found, do you want to overwrite it?")
                .interact()?
        {
            return Ok(Config::read()?);
        }

        Ok(Config::create_config()?)
    }

    pub fn create_config() -> Result<Config> {
        let configuration = Config::new()?;
        write_config(&configuration)?;
        Ok(configuration)
    }

    pub fn new() -> Result<Config> {
        let server_uri = query_server()?;
        let api_key = query_api_key()?;
        let project_name = query_project_name()?;
        let projects = search_for_project(&server_uri, &api_key, &project_name)?;
        let project_id = pick_a_project(&projects)?;
        let default_branch = query_default_branch()?;

        Ok(Config {
            server_uri,
            api_key,
            project_name,
            project_id,
            default_branch,
            ..Default::default()
        })
    }

    pub fn exists() -> bool {
        let path = get_config_path();
        match path {
            Ok(path) => path.exists(),
            Err(_) => false,
        }
    }

    pub fn read() -> Result<Config> {
        let path = get_config_path()?;
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents).map_err(|_| MargeError::ParseError)?;
        Ok(config)
    }
}

fn get_config_path() -> Result<PathBuf> {
    let path = git_path()?
        .parent()
        .ok_or(MargeError::PathNoParentError)?
        .join(".marge");

    Ok(path)
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

fn write_config(config: &Config) -> Result<()> {
    let path = get_config_path()?;
    let mut file = File::create(&path)?;
    let config_string = serde_yaml::to_string(&config).unwrap();
    Ok(file.write_all(config_string.as_bytes())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_remove_config() {
        write_config(&Config::default()).expect("erorr writing config file");
        assert!(Config::exists());

        let path = get_config_path().expect("config path not found");
        std::fs::remove_file(&path).expect("No permission to remove file");

        assert!(!Config::exists())
    }
}
