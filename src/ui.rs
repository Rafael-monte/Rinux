use std::collections::HashMap;
use std::io;
use chrono::Local;
use crate::command::traits::EssentialCommand;
use crate::core::InternalCommand;

pub struct PresentationLayer {
    pub history: Vec<String>,
    commands: InternalCommand
}

impl PresentationLayer {
    pub fn build(internal_modules: Vec<Box<dyn EssentialCommand>>) -> Self {
        let mut modules = HashMap::new();
        for internal_module in internal_modules {
            modules.insert(internal_module.get_name(), internal_module);
        }
        return Self {
            commands: modules,
            history: Vec::new()
        };
    }

    fn add_to_history(&mut self, command: &str, args: &[String]) -> () {
        const COMMA: &str = ",";
        let mut args_formatted = args.to_vec().join(COMMA);
        if args.to_vec().is_empty() || args.to_vec().iter().all(|entry| entry.is_empty()) {
            args_formatted = String::from("Empty");
        }
        let moment = Local::now();
        let entry = format!("Command: {}. Args=[{}] (at {})", command, args_formatted.as_str(), moment);
        println!("Entry: {}", entry);
        self.history.push(entry);
    }

    fn process_tokens(&self, input: &str) -> (String, Vec<String>) {
        let tokens: Vec<String> = input.split(" ").map(|el| el.to_string()).collect();
        let args: Vec<String> = tokens[1..].iter()
            .filter(|el| !el.is_empty())
            .map(|el| String::from(el.as_str()))
            .collect();
        return (tokens[0].to_string(), args);
    }

    pub fn watch_commands(&mut self) -> () {
        let exit_command = String::from("exit");
        let mut current_line: String = String::new();
        loop {
            io::stdin().read_line(&mut current_line)
                .expect("Occoured an error while reading the input line");
            current_line = current_line.replace("\n", "");
            if current_line == exit_command {
                break;
            }
            if current_line.is_empty() {
                current_line = String::new();
                continue;
            }
            let (command, args) = self.process_tokens(&current_line);
            self.add_to_history(&command, &args);
            if let Some(comm) = self.commands.get_mut(&command) {
                comm.check_and_run(&args);
            } else {
                eprintln!("Command \"{}\" not found.", command);
            }
            current_line = String::new();
        }
    }
}