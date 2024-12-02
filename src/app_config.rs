use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AppConfigReadError {
    NoConfigFile,
    DeserError(toml::de::Error),
}

#[derive(Debug)]
pub enum AppConfigGenError {
    ConfigFileReadError(toml::de::Error),
    ConfFileSerError(toml::ser::Error),
    ConfFileCreationError(std::io::Error),
    ConfFilePathError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub session_token_dir_path: String,
    pub templates_dir_path: String,
    pub hostname: String,
}

impl AppConfig {
    pub fn default_path() -> PathBuf {
        if let Some(d) = dirs::config_dir() {
            let mut path = d.clone();
            path.push("invoice_shelf_cli/InvoiceShelfCli.toml");
            return path;
        }

        panic!("Failed to get Config dir four your system!");
    }

    pub fn default() -> Self {
        let mut templates_path = Self::default_path();
        templates_path.pop();
        templates_path.push("templates/");

        let mut session_token_dir_path = Self::default_path();
        session_token_dir_path.pop();
        session_token_dir_path.push("session");

        if let None = templates_path.to_str() {
            panic!("Failed to instanciate a path for your templates!");
        }

        if let None = session_token_dir_path.to_str() {
            panic!("Failed to instanciate a path for your token!");
        }

        return Self {
            templates_dir_path: templates_path.to_str().unwrap().to_string(),
            hostname: String::from("https://your.server.com"),
            session_token_dir_path: session_token_dir_path.to_str().unwrap().to_string(),
        };
    }

    pub fn default_as_string() -> String {
        let res = toml::ser::to_string::<AppConfig>(&Self::default());
        if let Err(_) = res {
            panic!("The default error could not be serialized. THIS SHOULD NOT HAPPEN!");
        }

        return res.unwrap();
    }

    pub fn from_file(path: &str) -> Result<Self, AppConfigReadError> {
        let file_content: String;

        let contents = fs::read_to_string(path);

        if let Ok(conf_text) = contents {
            file_content = conf_text;
        } else {
            return Err(AppConfigReadError::NoConfigFile);
        }

        let conf = toml::from_str::<AppConfig>(&file_content);

        if let Err(e) = conf {
            return Err(AppConfigReadError::DeserError(e));
        }

        return Ok(conf.unwrap());
    }

    pub fn generate(path: &str, content: &str) -> Result<(), AppConfigGenError> {
        if let Err(e) = toml::from_str::<AppConfig>(content) {
            return Err(AppConfigGenError::ConfigFileReadError(e));
        }

        let dir = PathBuf::try_from(path);

        if let Err(_) = dir {
            return Err(AppConfigGenError::ConfFilePathError);
        }

        let u_dir = dir.unwrap().pop();

        if let Err(e) = fs::create_dir_all(u_dir.to_string()) {
            return Err(AppConfigGenError::ConfFileCreationError(e));
        }

        if let Err(e) = fs::write(path, content) {
            return Err(AppConfigGenError::ConfFileCreationError(e));
        }

        return Ok(());
    }
}
