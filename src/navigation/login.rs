use core::panic;

use inquire::{Password, Text};

use crate::{app_config::AppConfig, session::Session};

pub fn login_prompt(username: &Option<String>, password: &Option<String>, config: &AppConfig) {
    let username_str;

    if let None = username {
        let response = Text::new("Username:").prompt();

        if let Err(e) = response {
            panic!("Something wrong happened while interacting: {}", e);
        }

        username_str = response.unwrap();
    } else {
        username_str = String::from(username.clone().unwrap());
    }

    let password_str;

    if let None = password {
        let response = Password::new("Password:").prompt();

        if let Err(e) = response {
            panic!("Something wrong happened while interacting: {}", e);
        }

        password_str = response.unwrap();
    } else {
        password_str = String::from(password.clone().unwrap());
    }

    let session = Session::new(&username_str, &password_str, config);

    if let Err(e) = session {
        match e {
            crate::session::SessionOpeningError::WrongCreds(message) => {
                println!("Failed to login with message: {}", message);
                return login_prompt(&None, &None, config);
            }
            crate::session::SessionOpeningError::NetworkingError(e) => {
                panic!(
                    "Something wrong happened while contacting your sever :  {}",
                    e
                )
            }
            crate::session::SessionOpeningError::HttpError(e) => {
                panic!(
                    "Something wrong happened while contacting your sever :  {}",
                    e
                )
            }
        }
    }
}
