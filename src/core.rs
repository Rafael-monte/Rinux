use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::Path;
use chrono::{DateTime, Local};
use std::io::Write;
use crate::command::traits::{EssentialCommand};
use crate::core::internal_modules::get_internal_modules;
use crate::ui::PresentationLayer;

mod internal_modules;

pub type InternalCommand = HashMap<String, Box<dyn EssentialCommand>>;

pub struct Kernel {
    pub ui: PresentationLayer,
    session_time: DateTime<Local>
}

impl Kernel {
    pub fn bootloader() -> Self {
        let ui = PresentationLayer::build(get_internal_modules());
        return Self {
            ui,
            session_time: Local::now()
        }
    }

    pub fn get_module_in_protected_mode(module_name: &str) -> Option<Box<dyn EssentialCommand>> {
        for mut module in get_internal_modules() {
            if module_name == module.get_name() {
                module.set_protected_mode(true);
                return Some(module);
            }
        }
        return None;
    }

    pub fn run(&mut self) -> () {
        self.ui.watch_commands();
        self.save_history();
    }

    pub fn save_history(&mut self) -> () {
        const CURRENT_PATH: &str=".";
        const HISTORY_PATH: &str=".history";
        if std::fs::read_dir(Path::new(HISTORY_PATH)).is_err() {
          let _= std::fs::create_dir(Path::new(HISTORY_PATH)).unwrap();
        }
        let date = Local::now().to_utc();
        let f_name = format!("{}/session-{}.txt", HISTORY_PATH, self.session_time);
        let mut f = OpenOptions::new().create_new(true).write(true).open(f_name).unwrap();
        self.ui.history.iter().for_each(|entry| {
            writeln!(f, "{}", entry.as_str()).expect("Couldn't write to history file");
        })
    }
}