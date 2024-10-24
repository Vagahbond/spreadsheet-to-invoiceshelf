use std::collections::HashMap;

use calamine::{open_workbook, Data, Ods, OdsError, Range, Reader};

#[derive(Debug)]
pub enum ExcelReadingError {
    HeadersError(OdsError),
    InvalidAttributes(String),
    NoFirstPageError,
    ReadingError(OdsError),
}

pub type TaskList = HashMap<String, Vec<String>>;

fn read_column(range: &Range<Data>, column: u32) -> Option<(String, Vec<String>)> {
    let header = range.get((0, column.try_into().unwrap()));

    if let None = header {
        return None;
    }

    let mut values: Vec<String> = Vec::new();

    for line in 1..range.height() {
        let val = range.get((line, column.try_into().unwrap()));

        if let Some(v) = val {
            values.push(String::from(v.to_string()));
            /* if let Data::String(s_val) = v {
            }*/
        } else {
            values.push(String::from(""));
        }
    }

    return Some((String::from(header.unwrap().to_string()), values));
}

fn read_task_list_from_file(path: &str) -> Result<TaskList, ExcelReadingError> {
    let mut tasks = TaskList::new();

    let spreadsheet: Result<Ods<_>, _> = open_workbook(path);

    if let Err(e) = spreadsheet {
        return Err(ExcelReadingError::ReadingError(e));
    }

    let range = spreadsheet.unwrap().worksheet_range_at(0);

    if let None = range {
        return Err(ExcelReadingError::NoFirstPageError);
    }

    let cells = range.unwrap();

    if let Err(e) = cells {
        return Err(ExcelReadingError::HeadersError(e));
    }

    let cells = cells.unwrap();

    let width: u32 = cells.width().try_into().unwrap();

    for x in 0..width {
        let column = read_column(&cells, x);

        if let Some(c) = column {
            tasks.insert(c.0, c.1);
        }
    }

    return Ok(tasks);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args[1].is_empty() {
        println!("Please provide the path of the file you need to parse.");
        return;
    }

    let values = read_task_list_from_file(&args[1]);

    print!("{:?}", values);
}
