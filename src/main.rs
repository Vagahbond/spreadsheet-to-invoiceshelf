use std::process::ExitCode;

use clap::Parser;
use cli::Commands;
use navigation::{
    config_file_creation::create_config_file_prompt, import::import_prompt, login::login_prompt,
};

mod app_config;
mod cli;
mod navigation;
mod session;
mod spreadsheet_parsing;
mod template_mapping;

fn main() -> ExitCode {
    // Check args
    let args = cli::Args::parse();

    // Check for configuration file
    let mut parsed_conf = app_config::AppConfig::from_file(&args.config);

    if let Err(e) = &parsed_conf {
        match e {
            app_config::AppConfigReadError::NoConfigFile => {
                let new_conf = create_config_file_prompt(&args.config);
                parsed_conf = Ok(new_conf);
            }

            app_config::AppConfigReadError::DeserError(e) => {
                panic!("An error occured reading your Config: \n{}", e);
            }
        }
    }

    let command = args.command;

    match command {
        Commands::Login(args) => {
            login_prompt(&args.username, &args.password, &parsed_conf.unwrap())
        }
        Commands::Import(args) => import_prompt(&parsed_conf.unwrap(), &args),
    }
    return ExitCode::SUCCESS;
}
