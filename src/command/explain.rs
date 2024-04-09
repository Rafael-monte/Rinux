pub mod internal {
    use crate::command::traits::{EssentialCommand};
    use crate::core::Kernel;

    pub struct ExplainCommand {
        args: Vec<String>,
        protected_mode: bool
    }

    impl ExplainCommand {
        pub fn spawn() -> Self {
            return Self {
                args: Vec::new(),
                protected_mode: false
            }
        }
        fn get_command(&self, module_name: &str) -> Option<Box<dyn EssentialCommand>>  {
            return Kernel::get_module_in_protected_mode(module_name)
        }
    }

    impl EssentialCommand for ExplainCommand {
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

        fn run(&mut self, args: &[&str]) -> () {
            if self.protected_mode {
                eprintln!("Couldn't run command (protection mode enabled)");
                return;
            }
            self.args = args.iter().map(|arg| String::from(*arg)).collect();
            let command_name = args.first().unwrap();
            let opt_command = self.get_command(*command_name);
            if opt_command.is_none() {
                eprintln!("Couldn't find the command \"{}\"", command_name);
                return;
            }
            let command = opt_command.unwrap();
            println!();
            println!("-----------------------------------------------");
            command.description();
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

        fn set_protected_mode(&mut self, protected: bool) -> () {
            self.protected_mode = protected;
        }

        fn get_protected_mode(&self) -> bool {
            return self.protected_mode;
        }
    }
}