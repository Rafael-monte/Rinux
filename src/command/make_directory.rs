pub mod internal {
    use std::path::Path;
    use regex::Regex;
    use crate::command::traits::EssencialCommand;

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
    impl EssencialCommand for MakeDirectoryCommand {
        fn get_validators(&self) -> Vec<fn(&[&str]) -> bool> {
            return vec![
                |args| {
                    const NO_ARGS: usize = 0;
                    if args.len() == NO_ARGS {
                        eprintln!("Invalid Usage! (Expected at least 1 argument, found: 0)");
                    }
                    return args.len() == NO_ARGS;
                },
                |args| {
                    let invalid_names= false;
                    // TODO: Create a validator to check if the names bring by args are ok.
                    return invalid_names;
                }
            ]
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
            if validators.iter().any(|validation| validation(&slices)) {
                return;
            }
            self.args.iter().for_each(|dir_name| {
                let path = Path::new(dir_name);
                if *dir_name != self.get_name() {
                    if path.exists() && path.is_dir() {
                        eprintln!("The package \"{}\" already exists", path.display());
                        return;
                    }
                    std::fs::create_dir(dir_name).unwrap()
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