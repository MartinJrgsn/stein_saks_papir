use std::io;

use super::*;

pub struct TUI;

impl UI for TUI
{
    fn promt_for_name(self: &mut Self, is_valid: Option<&dyn Fn(&str) -> Option<NameError>>) -> String
    {
        let mut input = String::new();
        println!("Enter player name:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let name = input.trim().to_string();

        if let Some(is_valid_function) = is_valid
        {
            if let Some(error) = is_valid_function(&name)
            {
                match error
                {
                    NameError::Taken => println!("The name \"{}\" is already taken!", name),
                    NameError::Invalid => println!("The name \"{}\" is invalid!", name),
                    NameError::Other => ()
                }
                println!("Please choose a different name.");
                return self.promt_for_name(is_valid)
            }
        }

        name
    }
}