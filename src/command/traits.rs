use std::process::Command;
use regex::Regex;

pub trait SyscallInvoker {
    fn syscall(&self) -> Command;
    fn to_shell_command(&self) -> String;
}

pub trait EssentialCommand {
    fn get_validators(&self) -> Vec<fn(&[&str]) -> bool>;
    fn description(&self) -> () {
        // Template method
        let mut identifier: String = String::from("(⏏️ External)");
        if self.is_internal() {
            identifier = String::from("(🔽 Internal)");
        }
        println!("{} {} ({})", identifier, self.get_full_name(), self.get_name());
        println!("-----------------------------------------------");
        println!("Usage: {}", self.usage());
        println!("-----------------------------------------------");
        println!("🧑‍💻 Created by: {}", self.creator());
    }

    fn check_and_run(&mut self, args: &[String]) {
        let validators = self.get_validators();
        let words: Vec<&str> = args.iter().map(|arg| arg.as_str()).collect();
        if validators.iter().any(|invalid| invalid(&words)) {
            return;
        }
        if self.get_protected_mode() {
            eprintln!("Cannot run \"{}\" (Protection mode enabled)", self.get_name());
            return;
        }
        self.run(&words);
    }
    fn get_name(&self) -> String;
    fn usage(&self) -> String;
    fn run(&mut self, args: &[&str]) -> ();
    fn get_full_name(&self) -> String;
    fn is_internal(&self) -> bool;
    fn creator(&self) -> String;
    fn set_protected_mode(&mut self, protected:bool) -> ();
    fn get_protected_mode(&self) -> bool;
}

pub trait FileManipulatorValidations {
    fn name_validations(&self) -> Vec<fn(&[&str]) -> bool> {
        return vec![
            |args| {
                let file_and_directory_patterns = Regex::new(r#"^[a-zA-Z0-9._-]+$"#)
                    .unwrap();
                if args.iter().any(|arg| !file_and_directory_patterns.is_match(arg)) {
                    eprintln!("The provided name is invalid, please check the rules before create and try again");
                    return true;
                }
                return false;
            }
        ]
    }
}
