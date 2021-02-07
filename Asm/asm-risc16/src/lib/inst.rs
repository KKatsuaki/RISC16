use std::fmt;

use crate::lib::error::AsmError;
use crate::lib::risc16::*;

use Mnemonic::*;

#[derive(Debug)]
pub struct Instruction {
    mnemonic: Mnemonic,
    operand1: Option<u16>,
    operand2: Option<u16>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = self.into_u16();
        write!(f, "{:0>8x} {:0>8b}", code >> 8, code & 0xFF)
    }
}

impl Instruction {
    pub fn new(mnemonic: &Mnemonic, operand1: Option<u16>, operand2: Option<u16>) -> Self {
        Self {
            mnemonic: mnemonic.clone(),
            operand1,
            operand2,
        }
    }

    pub fn into_u16(&self) -> u16 {
        let ddd = match self.operand1 {
            Some(b) => b,
            None => 0,
        };

        let sss = match self.operand2 {
            Some(b) => b,
            None => 0,
        };

        let op = self.mnemonic.into_u16();

        match &self.mnemonic {
            // Jump Type
            JMP => {
                /* bitwise operation*/
                op | ddd
            }

            // Branch Type and Immediate Type
            BEQZ | BNEZ | BMI | BPL | ADDI | ANDI | ORI | LLI | LUI => {
                /* bitwise operation*/
                op | ddd << 8 | sss
            }

            // Memory Type and Register Type
            _ => {
                /* bitwise operation*/
                ddd << 8 | sss << 5 | op
            }
        }
    }
}

// convert negative number to 2's complement
pub fn conv_complement(n: i32, bits: usize) -> u16 {
    let n = if n < 0 { (n * (-1)) as u16 } else { n as u16 };
    (!n + 1) & (0b1111_1111_1111_1111) >> (16 - bits)
}
