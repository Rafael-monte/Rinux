use std::process::Command;

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