use std::io;
mod command;

fn main() {
    let exit_command = String::from("exit");
    let mut current_line: String = String::new();
    loop {
        io::stdin().read_line(&mut current_line)
            .expect("Occoured an error while reading the input line");
        current_line = current_line.replace("\n", "");
        if current_line == exit_command {
            break;
        }
        command::handle_command(&current_line);
        current_line = String::new();
    }
}
