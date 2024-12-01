use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fs};

use crate::spreadsheet_parsing::spreadsheet_data::TaskList;

#[derive(Debug)]
pub enum TemplateMappingError {
    FileOpeningError(std::io::Error),
    ParsingError(toml::de::Error),
    TemplateMappingError(String),
    NumberParsingError(String),
}

#[derive(Debug, Deserialize)]
pub struct TemplateMapping {
    invoice_name: String,
    template_name: String,
    inputs: HashMap<String, String>,
    outputs: TemplateMappingOutputs,
}

#[derive(Debug, Deserialize)]
pub struct TemplateMappingInputs(toml::Table);

#[derive(Debug, Deserialize)]
pub struct TemplateMappingOutputs {
    name: String,
    quantity: String,
    price: String,
    description: String,
    sub_total: String,
    total: String,
    unit_name: String,
}

#[derive(Debug)]
pub struct ComputedMappingOutput {
    pub name: String,
    pub quantity: i64,
    pub price: f64,
    pub description: String,
    pub sub_total: f64,
    pub total: f64,
    pub unit_name: String,
}

impl TemplateMapping {
    pub fn from_file(path: &str) -> Result<TemplateMapping, TemplateMappingError> {
        let contents = fs::read_to_string(path);

        if let Err(e) = contents {
            return Err(TemplateMappingError::FileOpeningError(e));
        }

        let template_mapping = toml::from_str::<TemplateMapping>(&contents.unwrap());

        if let Ok(m) = template_mapping {
            return Ok(m);
        }

        return Err(TemplateMappingError::ParsingError(
            template_mapping.unwrap_err(),
        ));
    }

    fn attr_name_regex() -> Regex {
        Regex::new(r"\$\{([^\$]*)\}").unwrap()
    }

    fn apply_line_number<T: std::str::FromStr>(
        &self,
        line: &str,
        data: &TaskList,
        index: usize,
    ) -> Result<T, TemplateMappingError> {
        let reg = Self::attr_name_regex();

        let mut found = reg.captures_iter(&line);

        let word = found.next();

        if found.next().is_some() {
            return Err(TemplateMappingError::TemplateMappingError(String::from(
                "It is not supported to use anything other than a simple value for numeric fields.",
            )));
        }

        let mut res_str = String::from(line);

        if let Some(w) = word {
            let col_token = &w[1];
            let col_name = self.inputs.get(col_token);

            if col_name.is_none() {
                return Err(TemplateMappingError::TemplateMappingError(
                    String::from("No input binding with specified name ") + col_token,
                ));
            }

            res_str = data.get(col_name.unwrap(), index).unwrap().to_string();
        }

        let as_number = str::parse::<T>(&res_str);

        if let Ok(v) = as_number {
            return Ok(v);
        } else {
            return Err(TemplateMappingError::NumberParsingError(res_str.clone()));
        }
    }

    fn apply_line_str(
        &self,
        line: &str,
        data: &TaskList,
        index: usize,
    ) -> Result<String, TemplateMappingError> {
        let reg = Self::attr_name_regex();
        let found = reg.captures_iter(&line);
        let mut res: String = String::from(line);

        for word in found {
            let w = &word[1];

            let column_name = self.inputs.get(w);

            if let None = column_name {
                return Err(TemplateMappingError::TemplateMappingError(
                    String::from("No input binding with specified name ") + w,
                ));
            }

            let str_c = column_name.unwrap();
            let task_value = data.get(&str_c, index);

            if let Err(_) = task_value {
                return Err(TemplateMappingError::TemplateMappingError(
                    String::from("Could not retrieve value from Excel : ") + &str_c,
                ));
            }

            let mut to_replace = String::from("${");
            to_replace.push_str(&w);
            to_replace.push_str("}");

            res = res.replacen(&to_replace, &task_value.unwrap().to_string(), 1);
        }

        return Ok(res);
    }

    pub fn apply(
        &self,
        list: &TaskList,
    ) -> Result<Vec<ComputedMappingOutput>, TemplateMappingError> {
        let mut res = Vec::<ComputedMappingOutput>::new();
        for index in 0..(list.len()) {
            let (name, quantity, price, description, sub_total, total, unit_name) = (
                self.apply_line_str(&self.outputs.name, list, index),
                self.apply_line_number::<i64>(&self.outputs.quantity, list, index),
                self.apply_line_number::<f64>(&self.outputs.price, list, index),
                self.apply_line_str(&self.outputs.description, list, index),
                self.apply_line_number::<f64>(&self.outputs.sub_total, list, index),
                self.apply_line_number::<f64>(&self.outputs.total, list, index),
                self.apply_line_str(&self.outputs.unit_name, list, index),
            );

            if name.is_err() {
                return Err(name.unwrap_err());
            }

            if quantity.is_err() {
                return Err(quantity.unwrap_err());
            }

            if price.is_err() {
                return Err(price.unwrap_err());
            }

            if description.is_err() {
                return Err(description.unwrap_err());
            }

            if sub_total.is_err() {
                return Err(sub_total.unwrap_err());
            }

            if total.is_err() {
                return Err(total.unwrap_err());
            }

            if unit_name.is_err() {
                return Err(unit_name.unwrap_err());
            }

            res.push(ComputedMappingOutput {
                name: name.unwrap(),
                quantity: quantity.unwrap(),
                price: price.unwrap(),
                description: description.unwrap(),
                sub_total: sub_total.unwrap(),
                total: total.unwrap(),
                unit_name: unit_name.unwrap(),
            })
        }
        return Ok(res);
    }
}

#[cfg(test)]
mod tests {
    use std::f64;

    use crate::spreadsheet_parsing::spreadsheet_data;

    use super::*;

    #[test]
    fn create_from_file() {
        let template_file_path = String::from("ExampleTemplate.toml");
        let mapping = TemplateMapping::from_file(&template_file_path).unwrap();

        assert_eq!(mapping.invoice_name, "ma_facture");
        assert_eq!(mapping.template_name, "Une facture pour ");

        assert_eq!(mapping.inputs.get("task").unwrap(), "Tâche");
        assert_eq!(mapping.inputs.get("time").unwrap(), "Durée");
        assert_eq!(mapping.inputs.get("cost").unwrap(), "Coût");

        assert_eq!(mapping.outputs.name, "${task} : ${time}");
        assert_eq!(mapping.outputs.quantity, "1");
        assert_eq!(mapping.outputs.price, "${cost}");
        assert_eq!(mapping.outputs.description, "${task}");
        assert_eq!(mapping.outputs.sub_total, "${cost}");
        assert_eq!(mapping.outputs.total, "${cost}");
        assert_eq!(mapping.outputs.unit_name, "Tâche");
    }

    #[test]
    fn template_regex() {
        let str_1 = "test test test";
        let str_2 = "test ${test} test";
        let str_3 = "test ${test} ${test";
        let str_4 = "test} ${test} ${test";
        let str_5 = "test} ${test} ${test2} fg";

        let cap_1 = TemplateMapping::attr_name_regex().captures(str_1);
        assert!(cap_1.is_none());

        let cap_2 = TemplateMapping::attr_name_regex().captures(str_2).unwrap();
        assert_eq!(cap_2.len(), 2);
        assert_eq!(&cap_2[0], "${test}");
        assert_eq!(&cap_2[1], "test");

        let cap_3 = TemplateMapping::attr_name_regex().captures(str_3).unwrap();
        assert_eq!(cap_3.len(), 2);
        assert_eq!(&cap_3[0], "${test}");
        assert_eq!(&cap_3[1], "test");

        let cap_4 = TemplateMapping::attr_name_regex().captures(str_4).unwrap();
        assert_eq!(cap_4.len(), 2);
        assert_eq!(&cap_4[0], "${test}");
        assert_eq!(&cap_4[1], "test");

        let cap_5 = TemplateMapping::attr_name_regex().captures(str_5).unwrap();
        assert_eq!(cap_5.len(), 2);
        assert_eq!(&cap_5[0], "${test}");
        assert_eq!(&cap_5[1], "test");
    }

    fn get_fake_task_list() -> TaskList {
        let mut task_hashmap = HashMap::<String, Vec<spreadsheet_data::Value>>::new();

        task_hashmap.insert(
            String::from("Test composé"),
            vec![
                spreadsheet_data::Value::String(String::from("some")),
                spreadsheet_data::Value::String(String::from("things")),
                spreadsheet_data::Value::String(String::from("in")),
                spreadsheet_data::Value::String(String::from("an")),
                spreadsheet_data::Value::String(String::from("array")),
            ],
        );

        task_hashmap.insert(
            String::from("Test Encore"),
            vec![
                spreadsheet_data::Value::String(String::from("yet")),
                spreadsheet_data::Value::String(String::from("other")),
                spreadsheet_data::Value::String(String::from("things")),
                spreadsheet_data::Value::String(String::from("in")),
                spreadsheet_data::Value::String(String::from("there")),
            ],
        );

        task_hashmap.insert(
            String::from("Test Floats"),
            vec![
                spreadsheet_data::Value::Float(1.1),
                spreadsheet_data::Value::Float(2.0),
                spreadsheet_data::Value::Float(3.3),
                spreadsheet_data::Value::Float(9.2),
                spreadsheet_data::Value::Float(10.0),
            ],
        );

        task_hashmap.insert(
            String::from("Test Ints"),
            vec![
                spreadsheet_data::Value::Integer(1),
                spreadsheet_data::Value::Integer(2),
                spreadsheet_data::Value::Integer(3),
                spreadsheet_data::Value::Integer(9),
                spreadsheet_data::Value::Integer(10),
            ],
        );
        return TaskList::from(task_hashmap);
    }

    fn get_fake_mapping() -> TemplateMapping {
        let mut inputs = HashMap::new();
        inputs.insert(String::from("tc"), String::from("Test composé"));
        inputs.insert(String::from("te"), String::from("Test Encore"));
        inputs.insert(String::from("tf"), String::from("Test Floats"));
        inputs.insert(String::from("ti"), String::from("Test Ints"));
        TemplateMapping {
            template_name: String::from("test template"),
            invoice_name: String::from("test invoice"),
            inputs,
            outputs: TemplateMappingOutputs {
                name: String::from("$nom ${tc}"),
                quantity: String::from("${ti}"),
                price: String::from("${tf}"),
                description: String::from("${tc} ${te}"),
                sub_total: String::from("${tf}"),
                total: String::from("${tf}"),
                unit_name: String::from("${te}${tc}"),
            },
        }
    }

    #[test]
    fn apply_line_str() {
        let mapping = get_fake_mapping();
        let task_list = get_fake_task_list();

        let line_1 = mapping.apply_line_str("${tc}", &task_list, 3).unwrap();
        assert_eq!(line_1, "an");

        let line_2 = mapping.apply_line_str("${tc} aaa", &task_list, 3).unwrap();
        assert_eq!(line_2, "an aaa");

        let line_3 = mapping
            .apply_line_str("${tc} ${te}", &task_list, 3)
            .unwrap();
        assert_eq!(line_3, "an in");

        let line_4 = mapping
            .apply_line_str(" some ${te} where ${tc} the ", &task_list, 3)
            .unwrap();
        assert_eq!(line_4, " some in where an the ");

        let line_5 = mapping.apply_line_str("yo", &task_list, 3).unwrap();
        assert_eq!(line_5, "yo");

        let line_6 = mapping
            .apply_line_str("yo ${te} yo ${te} yo ${te}", &task_list, 3)
            .unwrap();
        assert_eq!(line_6, "yo in yo in yo in");

        let line_7 = mapping.apply_line_str("yo", &task_list, 999).unwrap();
        assert_eq!(line_7, "yo");

        let line_8 = mapping.apply_line_str("", &task_list, 999).unwrap();
        assert_eq!(line_8, "");
    }

    #[test]
    fn apply_line_number() {
        let mapping = get_fake_mapping();
        let task_list = get_fake_task_list();

        let line_1 = mapping
            .apply_line_number::<f64>("${tf} a", &task_list, 3)
            .unwrap();
        assert_eq!(line_1, 9.2);

        let line_2 = mapping.apply_line_number::<f64>("${tf} ${ti}", &task_list, 3);
        assert!(line_2.is_err());

        let line_3 = mapping
            .apply_line_number::<f64>("${tf}", &task_list, 2)
            .unwrap();
        assert_eq!(line_3, 3.3);

        let line_4 = mapping
            .apply_line_number::<i64>("${ti}", &task_list, 3)
            .unwrap();
        assert_eq!(line_4, 9);

        let line_5 = mapping.apply_line_number::<f64>("yo", &task_list, 3);
        assert!(line_5.is_err());

        let line_6 = mapping.apply_line_number::<f64>("", &task_list, 3);
        assert!(line_6.is_err());
    }

    #[test]
    fn apply() {
        let mapping = get_fake_mapping();
        let task_list = get_fake_task_list();

        let items = mapping.apply(&task_list).unwrap();

        assert_eq!(items[0].name, "$nom some");
        assert_eq!(items[0].description, "some yet");
        assert_eq!(items[0].unit_name, "yetsome");
        assert_eq!(items[0].price, 1.1);
        assert_eq!(items[0].total, 1.1);
        assert_eq!(items[0].quantity, 1);
        assert_eq!(items[0].sub_total, 1.1);

        assert_eq!(items[1].name, "$nom things");
        assert_eq!(items[1].description, "things other");
        assert_eq!(items[1].unit_name, "otherthings");
        assert_eq!(items[1].price, 2.0);
        assert_eq!(items[1].total, 2.0);
        assert_eq!(items[1].quantity, 2);
        assert_eq!(items[1].sub_total, 2.0);

        assert_eq!(items[2].name, "$nom in");
        assert_eq!(items[2].description, "in things");
        assert_eq!(items[2].unit_name, "thingsin");
        assert_eq!(items[2].price, 3.3);
        assert_eq!(items[2].total, 3.3);
        assert_eq!(items[2].quantity, 3);
        assert_eq!(items[2].sub_total, 3.3);

        assert_eq!(items[3].name, "$nom an");
        assert_eq!(items[3].description, "an in");
        assert_eq!(items[3].unit_name, "inan");
        assert_eq!(items[3].price, 9.2);
        assert_eq!(items[3].total, 9.2);
        assert_eq!(items[3].quantity, 9);
        assert_eq!(items[3].sub_total, 9.2);

        assert_eq!(items[4].name, "$nom array");
        assert_eq!(items[4].description, "array there");
        assert_eq!(items[4].unit_name, "therearray");
        assert_eq!(items[4].price, 10.0);
        assert_eq!(items[4].total, 10.0);
        assert_eq!(items[4].quantity, 10);
        assert_eq!(items[4].sub_total, 10.0);
    }
}
