extern crate regex;

use std::error;
use std::fs::File;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub mod lib;

use lib::asm::Assembler;
use lib::env;

fn main() -> Result<()> {
    // get command line arguments
    let config = env::Config::new();

    // read input files
    let input_file = File::open(config.get_input_path())?;

    // porcess for output
    if config.is_on_stdout() {
        let mut assembler = Assembler::new(std::io::stdout(), input_file);
        assembler.assemble()?;
    } else {
        let mut assembler = Assembler::new(File::create(config.get_out_path())?, input_file);
        assembler.assemble()?;
    };

    Ok(())
}
