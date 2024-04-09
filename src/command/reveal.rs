pub mod internal {
    use std::ffi::OsStr;
    use std::fs::DirEntry;
    use std::io::ErrorKind;
    use std::path::{Display, Path, PathBuf};
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
        fn adjust_locus(&self, locus: &&str) -> Result<PathBuf, ErrorKind> {
            let complete_path = std::fs::canonicalize(*locus);
            if complete_path.is_err() {
                eprintln!("Couldn't construct the provided locus: \"{}\"", *locus);
                return Err(ErrorKind::InvalidData);
            }
            return Ok(complete_path.unwrap());
        }

        fn reveal_entry(&self, entry: &DirEntry) -> () {
            const FORMATTING_ERROR: &str = "(Couldn't format value)";
            let entry_p = entry.path();
            let is_directory = entry_p.is_dir();
            let mut prefix = "📦";
            let mut extension = OsStr::new("Package");
            if !is_directory {
                prefix = "📄";
                extension = entry_p.extension().unwrap_or(OsStr::new("Unknown"));
            }
            println!("{} {} - {}", prefix, entry.path().display(), extension.to_str().unwrap_or(FORMATTING_ERROR));
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
            const CURRENT_PATH: &str=".";
            let mut current_path = Path::new(CURRENT_PATH);
            let mut canonical_path = std::fs::canonicalize(current_path).unwrap();
            if let Some(locus) = args.first() {
                let complete_path = self.adjust_locus(locus);
                if complete_path.is_err() {
                    return;
                }
                canonical_path = complete_path.unwrap();
            }
            let _path_name_binding = canonical_path.clone();
            let path_name: Display<'_> = _path_name_binding.display();
            let directory = std::fs::read_dir(canonical_path);
            if directory.is_err() {
                eprintln!("Couldn't open directory \"{}\"", current_path.display());
                return;
            }
            println!("🔎 Looking into {}", path_name);
            for opt_entry in directory.unwrap() {
                if let Ok(entry) = opt_entry {
                    self.reveal_entry(&entry);
                }
            }
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