use std::{borrow::Borrow, fs};

use spreadsheet_parsing::spreadsheet_data::TaskList;
use template_mapping::TemplateMapping;

mod spreadsheet_parsing;
mod template_mapping;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide the path of the file you need to parse.");
        return;
    }

    let values = TaskList::try_from_path(&args[1]);

    if let Err(e) = values {
        print!(
            "An error occured while reading your spreadsheet file, {:?}",
            e
        );
        return;
    }

    println!("{:?}", values);

    let path = "/home/vagahbond/Projects/spreadsheet-to-invoiceshelf/ExampleTemplate.toml";
    let contents = fs::read_to_string(path).expect("Could not read file!");

    let template_mapping = toml::from_str::<TemplateMapping>(&contents);

    println!("{:?}", template_mapping);

    // let invoice_tasks = template_mapping.apply(values);
}
