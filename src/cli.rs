use clap::{Parser, Subcommand};

use crate::app_config::AppConfig;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Provide your own path for a configuration
    #[arg(
        short,
        long,
        default_value_t = String::from(AppConfig::default_path().to_str().unwrap())
    )]
    pub config: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Import an ODS file with a list of tasks as an invoice
    Import(ImportArgs),
    /// Login to InvoiceShelf
    Login(LoginArgs),
}

#[derive(Parser, Debug)]
pub struct ImportArgs {
    /// Path to the template file
    #[arg(short, long, default_value_t = String::from("./Template.toml"))]
    template: String,
}

#[derive(Parser, Debug)]
pub struct LoginArgs {
    /// Your username on InvoiceShelf
    #[arg(short, long)]
    username: String,

    /// Your password on InvoiceShelf
    #[arg(short, long)]
    password: String,
}
