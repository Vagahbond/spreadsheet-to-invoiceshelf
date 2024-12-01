use std::process::ExitCode;

use clap::Parser;
use cli::Commands;
use navigation::config_file_creation::create_config_file_prompt;

mod app_config;
mod cli;
mod invoice_shelf;
mod navigation;
mod spreadsheet_parsing;
mod template_mapping;

fn main() -> ExitCode {
    // Check args
    let args = cli::Args::parse();

    // Check for configuration file
    let conf = app_config::AppConfig::from_file(&args.config);

    if let Err(e) = conf {
        match e {
            app_config::AppConfigReadError::NoConfigFile => create_config_file_prompt(&args.config),

            app_config::AppConfigReadError::DeserError(e) => {
                panic!("An error occured reading your Config: \n{}", e);
            }
        }
    }

    let command = args.command;

    match command {
        Commands::Login(_) => println!("NOT IMPLETMENTED YET! Come back later."),
        Commands::Import(_) => println!("NOT IMPLETMENTED YET! Come back later."),
    }

    return ExitCode::SUCCESS;
}
