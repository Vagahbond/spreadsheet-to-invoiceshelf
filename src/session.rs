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
                parsed.unwrap().write_token();
                return Ok(());
            }
            StatusCode::UNPROCESSABLE_ENTITY => {
                return Err(SessionOpeningError::WrongCreds(u_resp.text().unwrap()))
            }
            _ => return Err(SessionOpeningError::HttpError(u_resp.text().unwrap())),
        }
    }

    fn write_token(&self) {}
    pub fn resume(session_path: &str) {}
}
