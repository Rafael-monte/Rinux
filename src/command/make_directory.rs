pub mod internal {
    use std::path::Path;
    use crate::command::traits::{EssencialCommand, FileManipulatorValidations};

    pub struct MakeDirectoryCommand {
        args: Vec<String>
    }

    impl MakeDirectoryCommand {
        pub fn spawn(args: &[&str]) -> Self {
            return Self {
                args: args.to_vec().iter().map(|el| el.to_string()).collect()
            }
        }
    }

    impl FileManipulatorValidations for MakeDirectoryCommand {}
    impl EssencialCommand for MakeDirectoryCommand {
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
                            eprintln!("The given name {} already exists", path.display());
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
        fn run(&self) -> () {
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
    }
}


mod test {
    use crate::command::make_directory::internal::MakeDirectoryCommand;
    use crate::command::traits::EssencialCommand;

    #[test]
    fn create_new_package() {
        let invalid_name="invalid paste";
        let valid_name="valid_package";
        let package_names = vec![invalid_name, valid_name];
        let command = MakeDirectoryCommand::spawn(&package_names);
        command.run();
    }
}