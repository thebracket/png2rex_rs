use crate::args::CommandLineError;
use colored::Colorize;

pub fn print_cli_error(e: CommandLineError) {
    match e {
        CommandLineError::NumberOfParameters => {
            println!(
                "{}",
                "Error: you must specify an input and output file".red()
            );
            println!("Correct usage: png2rex <pngfile> <rexfile>");
        }
        CommandLineError::FileNotFound(f) => {
            println!("{}: {}", "Error: File not found".red(), f.yellow());
        }
    }
}
