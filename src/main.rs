use colored::Colorize;
use std::env;
mod args;
use args::parse_args;
mod cli_error;
use cli_error::print_cli_error;
mod png_reader;
use png_reader::read_png;
mod rex_writer;
use rex_writer::make_rex;

/// png2rex
/// (c) 2021 Herbert Wolverson (Bracket Productions), all rights reserved.
/// MIT Licensed (see LICENSE file).
///
/// Usage png2rex <png_file> <rex_file>

fn main() {
    println!("{}", "png2rex 1.0".yellow());
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
    let params = parse_args(&args);
    match params {
        Ok(params) => {
            if let Ok(buf) = read_png(&params) {
                println!("{}. Image is {} x {}.", "Loaded input PNG".green(), buf.width, buf.height);
                if let Err(e) = make_rex(&params, &buf) {
                    println!("{:?}", e);
                } else {
                    println!("{}", "Image converted.".green());
                }
            } else {
                println!("{} : {}", "Unable to read PNG file".red(), params.input.yellow());
            }
        }
        Err(e) => print_cli_error(e),
    }
}
