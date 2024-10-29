use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fs};

use crate::spreadsheet_parsing::spreadsheet_data::TaskList;

#[derive(Debug)]
pub enum TemplateMappingError {
    FileOpeningError(std::io::Error),
    ParsingError(toml::de::Error),
    TemplateMappingError(String),
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

pub struct ComputedMappingOutput {
    name: String,
    quantity: u32,
    price: u32,
    description: String,
    sub_total: u32,
    total: u32,
    unit_name: String,
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

    fn apply_line(
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

            res = res.replacen(&to_replace, &task_value.unwrap(), 1);
        }

        return Ok(res);
    }

    /* pub fn apply(&mut self, list: TaskList) -> () {
        //Vec<ComputedMappingOutput> {
        // let mut res = Vec::<ComputedMappingOutput>::new();
        for index in 0..(list.len()) {
            println!(
                "{:?}",
                Self::fill_template_line(&self.outputs.name, &self.inputs, index, list)
                    .unwrap_or(String::from("NaaN au fromage"))
            )
            /* res.push(ComputedMappingOutput {
                name: ,
                quantity: ,
                price: ,
                description: ,
                sub_total: ,
                total: ,
                unit_name: ,
            })*/
        }
        //return vec![];
    } */
}

#[cfg(test)]
mod tests {
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
        let mut task_hashmap = HashMap::<String, Vec<String>>::new();

        task_hashmap.insert(
            String::from("Test composé"),
            vec![
                String::from("some"),
                String::from("things"),
                String::from("in"),
                String::from("an"),
                String::from("array"),
            ],
        );

        task_hashmap.insert(
            String::from("Test Encore"),
            vec![
                String::from("yet"),
                String::from("other"),
                String::from("things"),
                String::from("in"),
                String::from("there"),
            ],
        );

        return TaskList::from(task_hashmap);
    }

    fn get_fake_mapping() -> TemplateMapping {
        let mut inputs = HashMap::new();
        inputs.insert(String::from("tc"), String::from("Test composé"));
        inputs.insert(String::from("te"), String::from("Test Encore"));
        TemplateMapping {
            template_name: String::from("test template"),
            invoice_name: String::from("test invoice"),
            inputs,
            outputs: TemplateMappingOutputs {
                name: String::from("$nom ${tc}"),
                quantity: String::from("1"),
                price: String::from("1"),
                description: String::from("${tc} ${te}"),
                sub_total: String::from("1"),
                total: String::from("1"),
                unit_name: String::from("${te}${tc}"),
            },
        }
    }

    #[test]
    fn apply_line() {
        let mapping = get_fake_mapping();
        let task_list = get_fake_task_list();

        let line_1 = mapping.apply_line("${tc}", &task_list, 3).unwrap();
        assert_eq!(line_1, "an");

        let line_2 = mapping.apply_line("${tc} aaa", &task_list, 3).unwrap();
        assert_eq!(line_2, "an aaa");

        let line_3 = mapping.apply_line("${tc} ${te}", &task_list, 3).unwrap();
        assert_eq!(line_3, "an in");

        let line_4 = mapping
            .apply_line(" some ${te} where ${tc} the ", &task_list, 3)
            .unwrap();
        assert_eq!(line_4, " some in where an the ");

        let line_5 = mapping.apply_line("yo", &task_list, 3).unwrap();
        assert_eq!(line_5, "yo");

        let line_6 = mapping
            .apply_line("yo ${te} yo ${te} yo ${te}", &task_list, 3)
            .unwrap();
        assert_eq!(line_6, "yo in yo in yo in");

        let line_7 = mapping.apply_line("yo", &task_list, 999).unwrap();
        assert_eq!(line_7, "yo");
    }
}
