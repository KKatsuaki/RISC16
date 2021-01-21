extern crate regex;

use std::io::{Read, Write};
use std::fs::File;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub mod asm;
use asm::{env, risc16::*};

fn main() -> Result::<()>{
    // get command line arguments
    let config = env::Config::new();

    // read input files
    let mut input_file = File::open(config.get_input_path())?;
    let mut buf = Vec::new();
    input_file.read_to_end(&mut buf)?;
    let buf = String::from_utf8(buf)?;
    let lines : Vec::<String> = buf.split("\n").map(|s| String::from(s)).collect();
    let mut offset = 0;
    
    // porcess for output
    if config.is_on_stdout(){
        for line in lines{
            if line.len() > 0{
                let code = Instruction::asm2inst(line.as_ref());
                let disp = format!("{}",code.in_ascii(&mut offset));
                println!("{}",disp)
            }
        }
    }else{
        //  files                                               
        let mut out_file = File::create(config.get_out_path())?;
        for line in lines{                                                     
            if line.len() > 0{
                let code = Instruction::asm2inst(line.as_ref());               
                let disp = format!("{} //{}\n",code.in_ascii(&mut offset),line);
                out_file.write(disp.as_bytes())?;
            }                                                                  
        }                                                                      
    }

    Ok(())
}

