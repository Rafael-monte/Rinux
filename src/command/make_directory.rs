pub mod internal {
    use std::path::Path;
    use crate::command::traits::{EssentialCommand, FileManipulatorValidations};

    pub struct MakeDirectoryCommand {
        args: Vec<String>,
        protected_mode: bool
    }

    impl MakeDirectoryCommand {
        pub fn spawn() -> Self {
            return Self {
                args: Vec::new(),
                protected_mode: false
            }
        }
    }

    impl FileManipulatorValidations for MakeDirectoryCommand {}
    impl EssentialCommand for MakeDirectoryCommand {
        fn get_validators(&self) -> Vec<fn(&[&str]) -> bool> {
            let mut validators: Vec<fn(&[&str]) -> bool> = vec![
                |args| {
                    const NO_ARGS: usize = 0;
                    if args.len() == NO_ARGS {
                        eprintln!("Invalid Usage! (Expected at least 1 argument, found: 0)");
                    }
                    return args.len() == NO_ARGS;
                },
                |args| {
                    let mut exists = false;
                    args.iter().for_each(|arg| {
                        let mut path = Path::new(*arg);
                        if path.exists() && path.is_dir() {
                            eprintln!("The given package \"{}\" already exists", path.display());
                            exists = true;
                        }
                    });
                    return exists;
                }
            ];
            validators.extend(self.name_validations());
            return validators;
        }

        fn get_name(&self) -> String {
            return String::from("mkpkg");
        }
        fn usage(&self) -> String {
            return String::from("mkpkg [📦 Package names...]")
        }
        fn run(&mut self, args: &[String]) -> () {
            if self.protected_mode {
                eprintln!("Couldn't run command (protection mode enabled)");
                return;
            }
            self.args = args.to_vec();
            let validators = self.get_validators();
            let slices: Vec<&str> = self.args.iter().map(|arg| arg.as_str()).collect();
            if validators.iter().any(|validation_triggered| validation_triggered(&slices)) {
                return;
            }
            self.args.iter().for_each(|dir_name| {
                if *dir_name != self.get_name() {
                    let package_created = std::fs::create_dir(dir_name);
                    if package_created.is_err() {
                        eprintln!("An error occoured when create package: {}", package_created.unwrap_err());
                        return;
                    }
                    package_created.unwrap();
                }
            })
        }
        fn get_full_name(&self) -> String {
            return String::from("MaKe PacKaGe");
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
    }
}