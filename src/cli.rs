pub mod cli_manager {
    use clap::{arg, Command};
    use colored::*;
    use std::{io, io::Write};

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
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("remove")
                    .about("Remove a task")
                    .arg(arg!(<Name> "The name of the task"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("add_section")
                    .about("Add a section")
                    .arg(arg!(<Name> "The name of the section to add"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("remove_section")
                    .about("Remove a section")
                    .arg(arg!(<Name> "The name of the section to remove"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("select_section")
                    .about("Select a section")
                    .arg(arg!(<Name> "The name of the section to select"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("set_completed")
                    .about("Set whether a task is completed or not")
                    .arg(arg!(<Name> "The name of the task to change"))
                    .arg_required_else_help(true),
            )
            .subcommand(Command::new("list_sections").about("List out all the sections"));

        match command.get_matches().subcommand() {
            Some(("list", _sub)) => {
                if !backend.map.contains_key(&backend.current_section) {
                    println!("{}", "The current section does not exist! Please make a new section and select it.".red());
                    return;
                }

                println!(
                    "Section {}",
                    if backend.is_section_completed() {
                        backend.current_section.green()
                    } else {
                        backend.current_section.red()
                    }
                );
                for task in backend.get_tasks() {
                    println!(
                        "  {} - {}",
                        if task.1.completed {
                            task.0.green()
                        } else {
                            task.0.red()
                        },
                        task.1.description
                    );
                }
            }
            Some(("add", sub)) => {
                let name = sub.get_one::<String>("Name").map(|s| s.as_str()).unwrap();

                let description = sub
                    .get_one::<String>("Description")
                    .map(|s| s.as_str())
                    .unwrap();

                println!("{}", backend.add_task(name, description));
            }
            Some(("remove", sub)) => {
                let name = sub.get_one::<String>("Name").map(|s| s.as_str()).unwrap();

                println!("{}", backend.remove_task(name));
            }
            Some(("add_section", sub)) => {
                let name = sub.get_one::<String>("Name").map(|s| s.as_str()).unwrap();

                println!("{}", backend.add_section(name));
            }
            Some(("remove_section", sub)) => {
                let name = sub.get_one::<String>("Name").map(|s| s.as_str()).unwrap();

                println!("{}", backend.remove_section(name));
            }
            Some(("select_section", sub)) => {
                let name = sub.get_one::<String>("Name").map(|s| s.as_str()).unwrap();

                println!("{}", backend.select_section(name));
            }
            Some(("set_completed", sub)) => {
                let name = sub.get_one::<String>("Name").map(|s| s.as_str()).unwrap();

                if !backend.map.contains_key(&backend.current_section) {
                    println!("{}", "The current section does not exist! Please make a new section and select it.".red());
                    return;
                }

                let result: bool = loop {
                    print!("Is this task compelted (Y/n): ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();

                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read user input");

                    match input.trim().to_lowercase().as_str() {
                        "y" | "" => break true,
                        "n" => break false,
                        _ => continue,
                    }
                };

                println!("{}", backend.set_task_completion(name, result));
            }
            Some(("list_sections", _sub)) => {
                for section in &backend.map {
                    println!(
                        "Section {}",
                        if &backend.current_section == section.0 {
                            section.0.green()
                        } else {
                            section.0.red()
                        }
                    );
                }
            }
            _ => println!("Not implemented yet."),
        }
    }
}
