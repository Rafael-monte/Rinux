pub mod internal {
    use std::ffi::OsStr;
    use std::path::Path;
    use crate::command::traits::EssentialCommand;

    pub struct Reveal {
        protected_mode: bool,
        args: Vec<String>
    }

    impl Reveal {
        pub fn spawn() -> Self {
            return Self {
                protected_mode: false,
                args: Vec::new()
            }
        }
    }

    impl EssentialCommand for Reveal {
        fn get_validators(&self) -> Vec<fn(&[&str]) -> bool> {
            return vec![
                |args| {
                    const ARGS_SIZE: usize = 1;
                    if args.len() > ARGS_SIZE {
                        eprintln!("Usage:rvl <location> OR rvl");
                    }
                    return args.len() > ARGS_SIZE;
                }
            ]
        }

        fn get_name(&self) -> String {
            return String::from("rvl");
        }

        fn usage(&self) -> String {
            return String::from("\nrvl <locus> \n OR \nrvl");
        }

        fn run(&mut self, args: &[&str]) -> () {
            const FORMATTING_ERROR: &str = "(Couldn't format value)";
            let mut current_path = Path::new(".");
            let mut path_name = "current path";
            if let Some(locus) = args.first() {
                path_name = *locus;
                todo!("Locus not yet implemented");
            }
            if let Ok(directory) = std::fs::read_dir(current_path) {
                println!("🔎📦 Looking into {}", path_name);
                for opt_entry in directory {
                    if let Ok(entry) = opt_entry {
                        let entry_p = entry.path();
                        let is_directory = entry_p.is_dir();
                        let mut prefix = "📦";
                        let mut extension = OsStr::new("Directory");
                        if !is_directory {
                            prefix = "📄";
                            extension = entry_p.extension().unwrap_or(OsStr::new("Unknown"));
                        }
                        println!("{} {} - {}", prefix, entry.path().display(), extension.to_str().unwrap_or(FORMATTING_ERROR));
                    }
                }
                return;
            }
            eprintln!("Couldn't open directory \"{}\"", current_path.display());
        }

        fn get_full_name(&self) -> String {
            return String::from("ReVeaL");
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