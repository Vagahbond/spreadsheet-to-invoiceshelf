use std::path::PathBuf;

use crate::{
    app_config::AppConfig, cli::ImportArgs, navigation::login::login_prompt,
    spreadsheet_parsing::spreadsheet_data::TaskList, template_mapping::TemplateMapping,
};

pub fn import_prompt(app_config: &AppConfig, args: &ImportArgs) {
    let excel_path = PathBuf::from(&args.spreadsheet);

    if !excel_path.is_file() {
        panic!(
            "Invalid path provided for the spreadsheet! \n {}",
            excel_path.to_str().unwrap()
        );
    }

    let template_path = PathBuf::from(&args.template);

    if !template_path.is_file() {
        panic!(
            "Invalid path provided for the template! \n {}",
            template_path.to_str().unwrap()
        );
    }

    let session_path = PathBuf::from(&app_config.session_token_dir_path);

    if !session_path.is_file() {
        println!("It seems you are not loogged in, please log in now. ",);
        login_prompt(&None, &None, app_config);
    }

    let lines = TaskList::try_from_path(excel_path.to_str().unwrap());

    if let Err(e) = lines {
        panic!("Failed to read your spreadsheet: {:?}", e)
    }

    let mapping = TemplateMapping::from_file(template_path.to_str().unwrap());

    if let Err(e) = mapping {
        panic!("Failed to read your mapping configuration: {:?}", e);
    }

    let items = mapping.unwrap().apply(&lines.unwrap());

    if let Err(e) = items {
        panic!("Could not apply template: {:?}", e);
    }

    panic!("{:?}", items.unwrap());
    // Send the request with the data
}
