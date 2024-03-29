use std::collections::HashMap;
use std::io;
use crate::command::traits::EssentialCommand;
use crate::core::InternalCommand;

pub struct PresentationLayer {
    history: Vec<String>,
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

    fn restore_history(&mut self) -> () {
        //TODO read a text file with the history
    }

    fn process_tokens(&self, input: &str) -> (String, Vec<String>) {
        let tokens: Vec<String> = input.split(" ").map(|el| el.to_string()).collect();
        return (tokens[0].to_string(), tokens[1..].to_vec());
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
            let (command, args) = self.process_tokens(&current_line);
            self.history.push(current_line);
            self.commands.get_mut(&command).unwrap().run(&args);
            current_line = String::new();
        }
    }
}