use inquire::{Confirm, Editor};

use crate::app_config::{self, AppConfig};

pub fn create_config_file_prompt(config_path: &str) -> AppConfig {
    println!("No config file found at {}.", config_path);

    let create_answer = Confirm::new("Do you want to create a fresh one ?")
        .with_default(true)
        .with_help_message("It will contain default parameters")
        .prompt();

    if let Err(e) = create_answer {
        panic!(
            "An error occured while interacting with the terminal ! \n {}",
            e
        );
    }

    if !create_answer.unwrap() {
        panic!("Exiting...");
    }

    let mut config = AppConfig::default_as_string();

    let mut edit_prompt = "Please edit the default config to your liking.";
    loop {
        let edited_config = Editor::new(edit_prompt)
            .with_predefined_text(&config)
            .prompt();

        if let Err(e) = edited_config {
            panic!("An error occured  ! \n {}", e);
        }

        let u_edited_config = edited_config.unwrap();

        let parsed = app_config::AppConfig::generate(config_path, &u_edited_config);

        if let Err(e) = parsed {
            match e {
                app_config::AppConfigGenError::ConfFileSerError(e) => {
                    panic!("An error occured while creating the config ! \n {}", e);
                }
                app_config::AppConfigGenError::ConfigFileReadError(e) => {
                    println!("An error occured with the provided config : \n {}", e);
                    edit_prompt = "An error occured. Please retry!";
                    config = u_edited_config;
                }
                app_config::AppConfigGenError::ConfFileCreationError(e) => {
                    panic!("An error occured while creating the config ! \n {}", e);
                }
                app_config::AppConfigGenError::ConfFilePathError => {
                    panic!("Provided path is not valid.");
                }
            }
        } else {
            println!("Config file created successfully !");
            return parsed.unwrap();
        }
    }
}
