use crate::lib::error::AsmError;
use crate::lib::risc16::*;
use crate::lib::tokenizer::Token;

use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

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
            let buf = line.trim();
            tokens.push(Token::tokenize(&buf, &mut lnum, &mut self.label_map)?);
            line.clear();
        }

        for token in tokens {
            let code = self.token_handler(&token)?;
            match &code{
                Some(s) => {
                    let out = format!("{} //{}\n", s, tokens2str(token));
                    self.writer.write_all(out.as_bytes())?;
                    self.offset += 2;
                },
                None => ()
            }
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
                Token::SetLabel(_) | Token::Comment => None,

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
                        "@{:0>4x} {:0>8b} {:0>8b}",
                        addr,
                        data >> 8,
                        data & 0xff
                    ))
                }

                Token::Data(d) => Some(format!(
                    "@{:0>4x} {:0>8b} {:0>8b}",
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
                            Token::Data(d) => {
                                let bits = match mnemonic{
                                    Mnemonic::JMP => 11,
                                    _ => 8
                                };
                                let offset = if *d > 0 {word_align(*d as u16, bits)}else{conv_complement(*d as i32, bits)};
                                Some(offset)
                            },
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
                                        match mnemonic{
                                            Mnemonic::JMP =>  Some(conv_complement(offset, 11)),
                                            Mnemonic::BEQZ | Mnemonic::BNEZ | Mnemonic::BMI | Mnemonic::BPL => Some(conv_complement(offset, 8)),
                                            _ => return Err(AsmError::boxed())
                                        }
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
                            Token::Data(d) => {
                                let bits = match mnemonic{
                                    Mnemonic::JMP => 11,
                                    _ => 8
                                };
                                let offset = if *d > 0 {word_align(*d as u16, bits)}else{conv_complement(*d as i32, bits)};
                                Some(offset)
                            },
                            _ => {
                                println!("unexpected token");
                                return Err(AsmError::boxed());
                            }
                        },
                        None => None,
                    };

                    let code = Instruction::new(&mnemonic, op1, op2);
                    Some(format!("@{:0>4x} {}", self.offset, code))
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
fn tokens2str(tokens : Vec::<Token>) -> String{
    let mut res = String::new();
    for tok in tokens{
        res = format!("{} {}",res, tok);
    }
    res
}

fn conv_complement(n: i32, bits: usize) -> u16 {
    let n = if n < 0 { (n * (-1)) as u16 } else { n as u16 };
    let n = !n + 1;
    word_align(n, bits)
    //(!n + 1) & (0b1111_1111_1111_1111) >> (16 - bits)
}

fn word_align(n : u16, bits : usize) -> u16{
    n & ((0b1111_1111_1111_1111) >> (16 - bits))
}

