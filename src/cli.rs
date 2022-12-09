
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
            _ => println!("Not implemented yet."),
        }
    }
}
