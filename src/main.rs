use std::process::{exit, ExitCode};

use clap::Parser;
use cli::Commands;

mod app_config;
mod cli;
mod invoice_shelf;
mod spreadsheet_parsing;
mod template_mapping;

fn main() -> ExitCode {
    // Check args
    let args = cli::Args::parse();

    // Check for configuration file
    let conf = app_config::AppConfig::from_file(&args.config);

    if let Err(e) = conf {
        match e {
            app_config::AppConfigReadError::NoConfigFile => {
                println!(
                    "No config file found. Creating one with default config at {}.",
                    args.config
                );

                if let Err(e) = app_config::AppConfig::generate(&args.config) {
                    match e {
                        app_config::AppConfigGenError::ConfFileSerError(e) => {
                            println!("An error occured while creating the config ! \n {}", e);
                            return ExitCode::FAILURE;
                        }

                        app_config::AppConfigGenError::ConfFileCreationError(e) => {
                            println!("An error occured while creating the config ! \n {}", e);
                            return ExitCode::FAILURE;
                        }
                        app_config::AppConfigGenError::ConfFilePathError => {
                            println!("Provided path is not valid.");
                            return ExitCode::FAILURE;
                        }
                    }
                }
            }

            app_config::AppConfigReadError::DeserError(e) => {
                println!("An error occured reading your Config: \n{:?}", e);
            }
        }
    }

    let command = args.command;

    match command {
        Commands::Login(sub_args) => println!("args : {:?}", sub_args),
        Commands::Import(sub_args) => println!("args: {:?}", sub_args),
    }

    return ExitCode::SUCCESS;
}
