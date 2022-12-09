use std::collections::HashMap;
use std::{fs::File, io::Read, io::Write};
use xdg::BaseDirectories;
use ron::{de::from_str, ser::to_string, ser::PrettyConfig};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SectionManager {
    pub map: HashMap<String, Section>,
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
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, section_name: &str, task_name: &str, task_description: &str) {
        self.map.get_mut(section_name).unwrap().tasks.insert(
            task_name.to_string(),
            Task {
                description: task_description.to_string(),
                completed: false,
            }
        );
    }

    pub fn get_task_mut<'a>(
        &'a mut self,
        section_name: &'a str,
        task_name: &'a str,
    ) -> &'a mut Task {
        self.map
            .get_mut(section_name)
            .unwrap()
            .tasks
            .get_mut(task_name)
            .unwrap()
    }

    pub fn is_section_completed(&self, section_name: &str) -> bool {
        let mut completed: bool = true;

        for task in &self.map.get(section_name).unwrap().tasks {
            if !task.1.completed {
                completed = false;
            }
        }

        completed
    }
}
