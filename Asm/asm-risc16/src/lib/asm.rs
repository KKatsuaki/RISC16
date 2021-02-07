use crate::lib::error;
use crate::lib::error::AsmError;
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
        let mut tokens = Vec::new();
        let mut line = String::new();
        let mut lnum = 1;
        while self.reader.read_line(&mut line)? != 0 {
            tokens.push(Token::tokenize(&line, lnum, &mut self.label_map)?);
            lnum += 1;
            line.clear();
        }

        for token in tokens {
            println!("{:?} : {:?}", token, self.token_handler(&token).unwrap());
            self.offset += 2;
        }

        println!("\n\nHash Table:");

        for pair in self.label_map.iter() {
            println!("{:?}", pair);
        }

        Ok(())
    }

    /*
    INST : NOP
    | REG_T
    | IMM_T
    | BRANCH_T
    | JMP_T

    REG_T : MNEMONIC REG REG
    IMM_T : MNEMONIC REG DATA
    BRANCH_T : MNEMONIC REG DATA
    |MNEMONIC REG LAB
    JMP_T : MNEMONIC LAB
    |MNEMONIC DAAT

    MEM : ADDR DATA
    |DATA

    LAB_SET : LAB
     */

    fn token_handler(&mut self, tok: &Vec<Token>) -> Result<Option<String>> {
        let mut iter = tok.iter().peekable();

        let res = if let Some(tok) = iter.peek() {
            match tok {
                Token::SetLabel(_) | Token::Comment => {
                    self.offset -= if self.offset >= 2 { 2 } else { 0 };
                    None
                }

                Token::Addr(addr) => {
                    iter.next().unwrap(); // consume
                    self.offset -= 2;
                    let data = match iter.next() {
                        Some(d) => match d {
                            Token::Data(data) => data,
                            _ => return Err(AsmError::boxed()),
                        },
                        None => return Err(AsmError::boxed()),
                    };
                    Some(format!(
                        "@{:0>3x} {:0>8b} {:0>8b}",
                        addr,
                        data >> 8,
                        data & 0xff
                    ))
                }

                Token::Data(d) => Some(format!(
                    "@{:0>3x} {:0>8b} {:0>b}",
                    self.offset,
                    d >> 8,
                    d & 0xff
                )),

                Token::Mnemo(mnemonic) => {
                    iter.next(); // consume
                    let op1 = match iter.next() {
                        Some(_op) => match _op {
                            Token::Reg(reg) => Some(reg.reg2bin()),
                            Token::Label(lab) => match self.label_map.get(lab) {
                                Some(op) => {
                                    let offset = self.calc_offset(*op);
                                    if offset < 0 {
                                        Some(conv_complement(offset, 11))
                                    } else {
                                        if offset > 0xFFFF {
                                            println!("offset is larger than 0xffff.");
                                            return Err(AsmError::boxed());
                                        } else {
                                            Some(offset as u16)
                                        }
                                    }
                                }
                                None => {
                                    println!("{} isn't used..", lab);
                                    return Err(AsmError::boxed());
                                }
                            },
                            Token::Data(d) => Some(*d as u16),
                            _ => {
                                println!("unexpected token");
                                return Err(AsmError::boxed());
                            }
                        },
                        None => None,
                    };

                    let op2 = match iter.next() {
                        Some(_op) => match _op {
                            Token::Reg(reg) => Some(reg.reg2bin()),
                            Token::Label(lab) => match self.label_map.get(lab) {
                                Some(op) => {
                                    let offset = self.calc_offset(*op);
                                    if offset < 0 {
                                        Some(conv_complement(offset, 11))
                                    } else {
                                        if offset > 0xFFFF {
                                            println!("offset is larger than 0xffff.");
                                            return Err(AsmError::boxed());
                                        } else {
                                            Some(offset as u16)
                                        }
                                    }
                                }
                                None => {
                                    println!("{} isn't used..", lab);
                                    return Err(AsmError::boxed());
                                }
                            },
                            Token::Data(d) => Some(*d as u16),
                            _ => {
                                println!("unexpected token");
                                return Err(AsmError::boxed());
                            }
                        },
                        None => None,
                    };

                    let code = Instruction::new(&mnemonic, op1, op2);
                    Some(format!("@{:0>3x} {}", self.offset, code))
                }
                _ => return Err(AsmError::boxed()),
            }
        } else {
            None
        };

        Ok(res)
    }

    fn calc_offset(&self, dst: u16) -> i32 {
        let cur = self.offset as i32;
        let dst = dst as i32;
        if dst > cur {
            dst - (cur + 2)
        } else {
            dst - cur
        }
    }
}
