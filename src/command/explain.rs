pub mod internal {
    use std::collections::HashMap;
    use crate::command::traits::EssencialCommand;

    pub struct ExplainCommand {
        args: Vec<String>
    }

    impl ExplainCommand {
        pub fn spawn(args: &[&str]) -> Self {
            let as_string: Vec<String> = args.iter().map(|el| el.to_string()).collect();
            return Self {
                args: as_string
            }
        }
        fn get_other_commands(&self) -> HashMap<String, Box<dyn EssencialCommand>>  {
            return crate::command::generate_commands(&vec![]);
        }
    }

    impl EssencialCommand for ExplainCommand {
        fn get_validators(&self) -> Vec<fn(&[&str]) -> bool> {
            return vec![
                |args| {
                    const ARGS_LEN: usize = 1;
                    if args.len() != ARGS_LEN {
                        eprintln!("Couldn't parse arguments: Expected: 1, Found: {}", args.len());
                    }
                    return args.len() != ARGS_LEN;
                }
            ]
        }

        fn get_name(&self) -> String where Self: Sized {
            return String::from("explain")
        }

        fn usage(&self) -> String {
            return String::from("explain [📝COMMAND]");
        }

        fn run(&self) -> () {
            let commands = self.get_other_commands();
            let slices: Vec<&str> = self.args.iter().map(|arg| arg.as_str()).collect();
            let validators = self.get_validators();
            if validators.iter().any(|validation| validation(&slices)) {
                return;
            }
            let command_name = slices.first().unwrap();
            if !commands.contains_key(*command_name) {
                eprintln!("Couldn't find the command {}", command_name);
                return;
            }
            println!();
            println!("-----------------------------------------------");
            commands[*command_name].description();
            println!("-----------------------------------------------");
            println!();
        }

        fn get_full_name(&self) -> String {
            return String::from("Explain");
        }

        fn is_internal(&self) -> bool {
            return true;
        }

        fn creator(&self) -> String {
            return String::from("Rafael Monteiro Zancanaro");
        }
    }
}