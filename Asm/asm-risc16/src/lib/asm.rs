use crate::lib::error;
use crate::lib::inst::*;
use crate::lib::risc16::*;
use crate::lib::tokenizer::Token;

use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Assembler<W: Write, R: Read> {
    offset: u16,
    label_map: HashMap<String, u16>,
    writer: BufWriter<W>,
    reader: BufReader<R>,
}

impl<W, R> Assembler<W, R>
where
    W: Write,
    R: Read,
{
    pub fn new(writer: W, reader: R) -> Self {
        Self {
            offset: 0,
            label_map: HashMap::new(),
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
        }
    }

    pub fn assemble(&mut self) -> Result<()> {
        loop {
            let mut line = String::new();
            if self.reader.read_line(&mut line)? == 0 {
                break;
            } else {
                let tokens = Token::tokenize(&line)?;
                self.writer
                    .write_all(format!("{:?}\n", tokens).as_bytes())?;
                self.writer.flush();
            }
        }

        Ok(())
    }

    pub fn token_handler(tokens: Vec<Token>) -> String {
        String::new()
    }
}
