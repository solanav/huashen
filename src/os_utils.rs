use std::process::Command;
use std::str;
use std::io;

pub fn execute_command(s: &String) -> Result<String, io::Error> {
    // Split the command on spaces
    let command_list: Vec<&str> = s.split(" ").collect();
    
    // Add the program to run
    let mut command = Command::new(command_list[0]);

    // Add the arguments
    for arg in command_list {
        command.arg(arg);
    }

    // Execute the command
    let output = command.output()?;

    // Return a string with output
    Ok(str::from_utf8(&output.stdout)
        .expect("Failed to parse char")
        .to_string())
}