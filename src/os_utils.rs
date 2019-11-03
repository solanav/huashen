use std::process;
use std::str;

pub fn execute_command(s: String) -> String {
    // Split the command on spaces
    let command_list: Vec<&str> = s.split(" ").collect();
    
    // Add the program to run
    let mut command = process::Command::new(command_list[0]);

    // Add the arguments
    if command_list.len() > 1 {
        for arg in command_list {
            command.arg(arg);
        }
    }

    // Execute the command
    let output = command.output()
        .expect("Failed to execute command");

    // Return a string with output
    str::from_utf8(&output.stdout)
        .expect("Failed to parse char")
        .to_string()
}