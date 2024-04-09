use crate::core::Kernel;

mod command;
mod core;
mod ui;

fn main() {
    let mut kernel = Kernel::bootloader();
    kernel.run();
}
