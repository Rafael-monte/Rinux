use crate::command::explain::internal::ExplainCommand;
use crate::command::make_directory::internal::MakeDirectoryCommand;
use crate::command::reveal::internal::Reveal;
use crate::command::traits::EssentialCommand;

pub fn get_internal_modules() -> Vec<Box<dyn EssentialCommand>> {
    return vec! [
        Box::new(MakeDirectoryCommand::spawn()) as Box<dyn EssentialCommand>,
        Box::new(ExplainCommand::spawn()) as Box<dyn EssentialCommand>,
        Box::new(Reveal::spawn()) as Box<dyn EssentialCommand>
    ]
}