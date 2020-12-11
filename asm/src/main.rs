extern crate regex;

use std::io::Read;
use std::fs::File;
use std::env;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub mod risc16;
use risc16::*;

fn main() -> Result::<()>{
    let mut input_file = match env::args().nth(1){
        Some(s) => File::open(s)?,
        None => panic!("Too few arguments...")
    };

    let mut buf = Vec::new();
    input_file.read_to_end(&mut buf)?;
    let buf = String::from_utf8(buf)?;
    let lines : Vec::<String> = buf.split("\n").map(|s| String::from(s)).collect();
    let mut offset = 0;
    for line in lines{
	if line.len() > 0{
	    let tmp = Instruction::asm2inst(line.as_ref());
	    println!("@{:0>2x} {} // {}",offset, tmp.in_ascii(),tmp);
	    offset+=2;
	}
    }
    Ok(())
}
