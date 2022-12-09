use std::collections::HashMap;
use std::{fs::File, io::Read, io::Write, path::Path};
use xdg::BaseDirectories;
use ron::{de::from_str, ser::to_string};
use serde::{Serialize, Deserialize};
use colored::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SectionManager {
    pub map: HashMap<String, Section>,
    pub current_section: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    pub tasks: HashMap<String, Task>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl SectionManager {
    pub fn load_from_file() -> Self {
        let xdg_dirs = BaseDirectories::with_prefix("tea_todo").unwrap();

        let path = Path::new("")
            .join(xdg_dirs.get_data_home())
            .join("lists.ron");

        let c_path = path.clone();

        let mut data = String::new();
        if c_path.exists() {
            let mut data_file = File::open(path).expect("Failed to open the data file.");

            data_file
                .read_to_string(&mut data)
                .expect("Failed to read the data.");

            from_str(&data).unwrap()
        } else {
            Self {
                map: HashMap::new(),
                current_section: "".to_string(),
            }
        }
    }

    pub fn save_to_file(&self) {
        let data = to_string(&self).unwrap();

        let conf = BaseDirectories::place_data_file(
            &BaseDirectories::with_prefix("tea_todo").unwrap(),
            "lists.ron"
        )
        .expect("Cannot create the configurationd directory.");

        let mut conf = File::create(conf).unwrap();

        conf.write_all(data.as_bytes())
            .expect("Failed to write data.");
    }

    pub fn add_task(&mut self, task_name: &str, task_description: &str) -> String {
        if !self.map.get(&self.current_section).unwrap().tasks.contains_key(task_name) {
            self.map.get_mut(&self.current_section).unwrap().tasks.insert(task_name.to_string(), Task { description: task_description.to_string(), completed: false } );
            format!("Added task {}", task_name.green())
        } else { String::from("Task already exists.").red().to_string() }
    }

    pub fn set_task_completion(&mut self, task_name: &str, completion: bool) -> String {
        if self.map.get(&self.current_section).unwrap().tasks.contains_key(task_name) {
            self.map.get_mut(&self.current_section).unwrap().tasks.get_mut(task_name).unwrap().completed = completion;
            format!("Setting task {} to {}", task_name.green(), if completion { "Completed" } else { "Uncompleted" })
        } else {
            String::from("Task doesn't exist.").red().to_string()
        }
    }

    pub fn is_section_completed(&self) -> bool {
        let mut completed: bool = true;

        for task in &self.map.get(&self.current_section).unwrap().tasks {
            if !task.1.completed {
                completed = false;
            }
        }

        completed
    }

    pub fn select_section(&mut self, section_name: &str) -> String {
        if self.map.contains_key(section_name) {
            self.current_section = section_name.to_string();
            format!("Selected section {}", section_name.green())
        } else { String::from("Section does not exist").red().to_string() }
    }

    pub fn add_section(&mut self, section_name: &str) -> String {
        if !self.map.contains_key(section_name) {
            self.map.insert(section_name.to_string(), Section { tasks: HashMap::new() });
            format!("Added section {}", section_name.green())
        } else { String::from("Section already exists.").red().to_string() }
    }

    pub fn remove_section(&mut self, section_name: &str) -> String {
        if self.map.contains_key(section_name) {
            self.map.remove(section_name);
            format!("Removed section {}", section_name.green())
        } else { String::from("Section does not exist.").red().to_string() }
    }

    pub fn remove_task(&mut self, task_name: &str) -> String {
        if self.map.contains_key(&self.current_section) {
            if self.map.get(&self.current_section).unwrap().tasks.contains_key(task_name) {
                self.map.get_mut(&self.current_section).unwrap().tasks.remove(task_name);
                format!("Removed task {}", task_name.green())
            } else { String::from("Task does not exist.") }
        } else { String::from("Section does not exist.").red().to_string() }
    }

    pub fn get_tasks(&'_ self) -> &'_ HashMap<String, Task> {
        &self.map.get(&self.current_section).unwrap().tasks
    }
}
