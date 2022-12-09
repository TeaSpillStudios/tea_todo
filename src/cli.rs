
pub mod cli_manager {
    use clap::{arg, Command, Subcommand};
    use colored::*;

    use crate::backend::SectionManager;

    pub fn start(backend: &mut SectionManager) {
        let command = Command::new("TeaToDo")
            .about("A CLI ToDo app.")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("list").about("Show all tasks"))
            .subcommand(
                Command::new("add")
                    .about("Add a new task")
                    .arg(arg!(<Name> "The name of the task"))
                    .arg(arg!(<Description> "The description of the task"))
                    .arg_required_else_help(true)
            )
            .subcommand(
                Command::new("remove")
                    .about("Remove a task")
                    .arg(arg!(<Name> "The name of the task"))
                    .arg_required_else_help(true)
            )
            .subcommand(
                Command::new("add_section")
                    .about("Add a section")
                    .arg(arg!(<Name> "The name of the section to add"))
                    .arg_required_else_help(true)
            )
            .subcommand(
                Command::new("remove_section")
                    .about("Remove a section")
                    .arg(arg!(<Name> "The name of the section to remove"))
                    .arg_required_else_help(true)
            )
            .subcommand(
                Command::new("select_section")
                    .about("Select a section")
                    .arg(arg!(<Name> "THe name of the section to select"))
                    .arg_required_else_help(true)
            );

        match command.get_matches().subcommand() {
            Some(("list", _sub)) => {
                println!("Section {}", if backend.is_section_completed() { backend.current_section.green() } else { backend.current_section.red() });
                for task in backend.get_tasks() {
                    println!("  {} - {}", if task.1.completed { task.0.green() } else { task.0.red() }, task.1.description);
                }
                println!("\n");
            }
            Some(("add", sub)) => {
                let name = sub
                    .get_one::<String>("Name")
                    .map(|s| s.as_str())
                    .unwrap();

                let description = sub
                    .get_one::<String>("Description")
                    .map(|s| s.as_str())
                    .unwrap();

                println!("{}", backend.add_task(name, description));
            }
            Some(("remove", sub)) => {
                let name = sub
                    .get_one::<String>("Name")
                    .map(|s| s.as_str())
                    .unwrap();

                println!("{}", backend.remove_task(name));
            }
            Some(("add_section", sub)) => {
                let name = sub
                    .get_one::<String>("Name")
                    .map(|s| s.as_str())
                    .unwrap();

                println!("{}", backend.add_section(name));
            }
            Some(("remove_section", sub)) => {
                let name = sub
                    .get_one::<String>("Name")
                    .map(|s| s.as_str())
                    .unwrap();

                backend.remove_section(name);
            }
            Some(("select_section", sub)) => {
                let name = sub
                    .get_one::<String>("Name")
                    .map(|s| s.as_str())
                    .unwrap();

                backend.select_section(name);
            }
            _ => println!("Not implemented yet."),
        }
    }
}
