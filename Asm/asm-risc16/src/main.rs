extern crate regex;

use std::error;
use std::fs::File;
use std::io::{Read, Write};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub mod lib;

use lib::asm::Assembler;
use lib::env;

fn main() -> Result<()> {
    // get command line arguments
    let config = env::Config::new();

    // read input files
    let mut input_file = File::open(config.get_input_path())?;

    // porcess for output
    //    let mut assembler = if config.is_on_stdout() {
    Assembler::new(std::io::stdout(), input_file).assemble()?;

    //    } else {
    //        Assembler::new(File::create(config.get_out_path())?, input_file);
    //    };

    Ok(())
}
