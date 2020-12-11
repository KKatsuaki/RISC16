use std::io::{self, Read};
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
    let mut buf = String::from_utf8(buf)?;
    let tokens : Vec::<String> = buf.split("\n").map(|s| String::from(s)).collect();

    
    
    
    
    Ok(())
}
