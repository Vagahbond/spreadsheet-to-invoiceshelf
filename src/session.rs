use std::{fs, path::PathBuf};

use reqwest::StatusCode;

use crate::app_config::AppConfig;

#[derive(serde::Deserialize)]
pub struct Session {
    token: String,
}

#[derive(serde::Serialize)]
struct LoginBody {
    username: String,
    password: String,
    device_name: String,
}

#[derive(Debug)]
pub enum SessionOpeningError {
    NetworkingError(reqwest::Error),
    WrongCreds(String),
    HttpError(String),
    WriteError(TokenWriteReadError),
}

#[derive(Debug)]
pub enum TokenWriteReadError {
    TokenPathResolveError,
    TokenFileCreationError(std::io::Error),
    TokenDirCreationError(std::io::Error),
}

impl Session {
    pub fn new(
        username: &str,
        password: &str,
        config: &AppConfig,
    ) -> Result<(), SessionOpeningError> {
        let http_client = reqwest::blocking::Client::new();

        let resp = http_client
            .post(String::from(&config.hostname) + "/api/v1/auth/login")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("company", "1")
            .json(&LoginBody {
                username: String::from(username),
                password: String::from(password),
                device_name: String::from("InvoiceShelfCli"),
            })
            .send();

        if let Err(e) = resp {
            return Err(SessionOpeningError::NetworkingError(e));
        }

        let u_resp = resp.unwrap();

        match u_resp.status() {
            StatusCode::OK => {
                println!("Successfully Logged in !");
                let parsed = u_resp.json::<Session>();

                if let Err(_) = parsed {
                    panic!("Failed to parse json response !!");
                }

                if let Err(e) = parsed.unwrap().write_token(&config.session_token_dir_path) {
                    return Err(SessionOpeningError::WriteError(e));
                }

                return Ok(());
            }
            StatusCode::UNPROCESSABLE_ENTITY => {
                return Err(SessionOpeningError::WrongCreds(u_resp.text().unwrap()))
            }
            _ => return Err(SessionOpeningError::HttpError(u_resp.text().unwrap())),
        }
    }

    fn write_token(&self, session_path: &str) -> Result<(), TokenWriteReadError> {
        let dir = PathBuf::try_from(session_path);

        if let Err(_) = dir {
            return Err(TokenWriteReadError::TokenPathResolveError);
        }

        let u_dir = dir.unwrap().pop();

        if let Err(e) = fs::create_dir_all(u_dir.to_string()) {
            return Err(TokenWriteReadError::TokenDirCreationError(e));
        }

        if let Err(e) = fs::write(session_path, self.token.clone().to_string()) {
            return Err(TokenWriteReadError::TokenFileCreationError(e));
        }
        return Ok(());
    }

    pub fn resume(session_path: &str) {}
}
