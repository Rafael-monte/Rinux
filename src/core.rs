use std::collections::HashMap;
use crate::command::traits::{EssentialCommand};
use crate::core::internal_modules::get_internal_modules;
use crate::ui::PresentationLayer;

mod internal_modules;

pub type InternalCommand = HashMap<String, Box<dyn EssentialCommand>>;

pub struct Kernel {
    pub ui: PresentationLayer
}

impl Kernel {
    pub fn bootloader() -> Self {
        let ui = PresentationLayer::build(get_internal_modules());
        return Self {
            ui
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
}