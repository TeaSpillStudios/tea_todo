use std::collections::HashMap;
use std::{fs::File, io::Read, io::Write, path::Path};
use xdg::BaseDirectories;
use ron::{de::from_str, ser::to_string};
use serde::{Serialize, Deserialize};

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
    pub fn load_from_file(&self) -> Self {
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

            Self {
                map: from_str(&data).unwrap(),
                current_section: "".to_string(),
            }
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

    pub fn select_section(&mut self, section_name: &str) -> bool {
        if self.map.contains_key(section_name) {
            self.current_section = section_name.to_string();
            true
        } else { false }
    }
}
