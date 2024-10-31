pub mod spreadsheet_data {
    use calamine::{open_workbook, Data, Ods, OdsError, Range, Reader};
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum SpreadsheetReadingError {
        HeadersError(OdsError),
        InvalidAttributes(String),
        NoFirstPageError,
        ReadingError(OdsError),
        AccessError(String),
    }

    #[derive(Debug)]
    pub enum Value {
        String(String),
        Float(f64),
        Integer(i64),
        None,
    }

    impl Value {
        pub fn to_string(&self) -> String {
            match self {
                Value::String(s) => s.clone(),
                Value::Float(f) => f.to_string(),
                Value::Integer(i) => i.to_string(),
                Value::None => String::from(""),
            }
        }
    }

    impl Clone for Value {
        fn clone(&self) -> Self {
            match self {
                Value::String(s) => Value::String(s.clone()),
                Value::Float(f) => Value::Float(f.clone()),
                Value::Integer(i) => Value::Integer(i.clone()),
                Value::None => Value::None,
            }
        }
    }

    #[derive(Debug)]
    pub struct TaskList(HashMap<String, Vec<Value>>);

    impl TaskList {
        pub fn from(map: HashMap<String, Vec<Value>>) -> Self {
            return Self(map);
        }

        pub fn try_from_path(path: &str) -> Result<Self, SpreadsheetReadingError> {
            let mut tasks = HashMap::<String, Vec<Value>>::new();

            let spreadsheet: Result<Ods<_>, _> = open_workbook(path);

            if let Err(e) = spreadsheet {
                return Err(SpreadsheetReadingError::ReadingError(e));
            }

            let range = spreadsheet.unwrap().worksheet_range_at(0);

            if let None = range {
                return Err(SpreadsheetReadingError::NoFirstPageError);
            }

            let cells = range.unwrap();

            if let Err(e) = cells {
                return Err(SpreadsheetReadingError::HeadersError(e));
            }

            let cells = cells.unwrap();

            let width: u32 = cells.width().try_into().unwrap();

            for x in 0..width {
                let column = Self::read_column(&cells, x);

                if let Some(c) = column {
                    tasks.insert(c.0, c.1);
                }
            }

            return Ok(TaskList(tasks));
        }

        fn spreadsheet_to_value(d: &Data) -> Value {
            match d {
                Data::String(s) => Value::String(String::from(s)),
                Data::Int(i) => Value::Integer(i.clone()),
                Data::Float(f) => Value::Float(f.clone()),
                Data::Bool(b) => Value::String(b.to_string()),
                Data::DateTime(dt) => Value::String(dt.to_string()),
                Data::DateTimeIso(dti) => Value::String(dti.to_string()),
                Data::DurationIso(di) => Value::String(di.to_string()),
                Data::Error(_) => Value::None,
                Data::Empty => Value::None,
            }
        }

        fn read_column(range: &Range<Data>, column: u32) -> Option<(String, Vec<Value>)> {
            let header = range.get((0, column.try_into().unwrap()));

            if let None = header {
                return None;
            }

            let mut values: Vec<Value> = Vec::new();

            for line in 1..range.height() {
                let val = range.get((line, column.try_into().unwrap()));

                if let Some(v) = val {
                    let converted = Self::spreadsheet_to_value(v);
                    if let Value::None = converted {
                        println!("Breaking at {:?} - {:?}", line, column);
                        break;
                    }
                    values.push(converted);
                } else {
                    values.push(Value::String(String::from("")));
                }
            }

            return Some((String::from(header.unwrap().to_string()), values));
        }

        pub fn len(&self) -> usize {
            let tasks = &self.0;

            let mut len = std::usize::MAX;
            for v in tasks.values() {
                if len > v.len() {
                    len = v.len();
                }
            }

            return len;
        }

        pub fn get(&self, header: &str, index: usize) -> Result<Value, SpreadsheetReadingError> {
            if let Some(values) = self.0.get(header) {
                if index > values.len() {
                    return Err(SpreadsheetReadingError::AccessError(String::from(
                        "Trying to read out of bounds of the spreadsheet's data.",
                    )));
                }

                return Ok(values[index].clone());
            }

            return Err(SpreadsheetReadingError::AccessError(
                String::from("Spreadsheet has no data under the name: ") + header,
            ));
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn placeholder() {
        assert_eq!(2, 1 + 1);
    }
}
