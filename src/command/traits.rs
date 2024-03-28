use std::process::Command;
use regex::Regex;

pub trait SyscallInvoker {
    fn syscall(&self) -> Command;
    fn to_shell_command(&self) -> String;
}

pub trait EssencialCommand {
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
    fn get_name(&self) -> String;
    fn usage(&self) -> String;
    fn run(&self) -> ();
    fn get_full_name(&self) -> String;
    fn is_internal(&self) -> bool;
    fn creator(&self) -> String;
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

mod test {
    use regex::Regex;

    #[test]
    pub fn name_validations() {
        let file_and_directory_patterns = Regex::new(r#"^[a-zA-Z0-9._-]+$"#)
            .unwrap();
        let invalid_name="invalid paste";
        let valid_name="valid_package";
        let package_names = vec![invalid_name, valid_name];
        let filter: Vec<_>= package_names.iter().filter(|p| !file_and_directory_patterns.is_match(**p)).collect();
        println!("\"{}\" name is invalid...", filter.first().unwrap());
        assert!(!filter.is_empty());
    }
}