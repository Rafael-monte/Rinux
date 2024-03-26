use std::collections::HashMap;
use crate::command::explain::internal::ExplainCommand;
use crate::command::make_directory::internal::MakeDirectoryCommand;
use crate::command::traits::EssencialCommand;

pub mod make_directory;
mod traits;
mod explain;

pub fn generate_commands(args: &[String]) -> HashMap<String, Box<dyn EssencialCommand>> {
    let slices: Vec<&str> = args.iter().map(|arg| arg.as_str()).collect();
    let mut table = HashMap::new();
    let commands = vec![
        Box::new(MakeDirectoryCommand::spawn(&slices)) as Box<dyn EssencialCommand>,
        Box::new(ExplainCommand::spawn(&slices)) as Box<dyn EssencialCommand>
    ];
    for command in commands {
        table.insert(command.get_name(), command);
    }
    return table;
}
pub fn handle_command(line: &str) -> () {
    let tokens: Vec<String> = line.split(" ").map(|el| el.to_string()).collect();
    let (command, args) = (tokens[0].as_str(), tokens[1..].to_vec());
    let commands = generate_commands(&args);
    if !commands.contains_key(command) {
        eprintln!("Command \"{}\" not found.", command);
        return;
    }
    commands[command].run();
}