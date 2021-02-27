use crate::lib::error::AsmError;
use crate::lib::result::Result;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Mnemonic {
    NOP,
    MV,
    NOT,
    XOR,
    ADD,
    SUB,
    SL,
    SR,
    AND,
    OR,
    ST,
    LD,
    ADDI,
    ANDI,
    ORI,
    LLI,
    LUI,
    BNEZ,
    BEQZ,
    BMI,
    BPL,
    JMP,
    SBU,
    LBU,
    SRB,
    SLB,
    SRHB,
    LL,
}

impl FromStr for Mnemonic {
    type Err = AsmError;
    fn from_str(s: &str) -> Result<Self> {
        let tmp = String::from(s);
        tmp.to_ascii_uppercase();
        match tmp.as_ref() {
            "NOP" => Ok(Self::NOP),
            "MV" => Ok(Self::MV),
            "NOT" => Ok(Self::NOT),
            "XOR" => Ok(Self::XOR),
            "ADD" => Ok(Self::ADD),
            "SUB" => Ok(Self::SUB),
            "SL" => Ok(Self::SL),
            "SR" => Ok(Self::SR),
            "AND" => Ok(Self::AND),
            "OR" => Ok(Self::OR),
            "ST" => Ok(Self::ST),
            "LD" => Ok(Self::LD),
            "LBU" => Ok(Self::LBU),
            "SBU" => Ok(Self::SBU),
            "ADDI" => Ok(Self::ADDI),
            "ANDI" => Ok(Self::ANDI),
            "ORI" => Ok(Self::ORI),
            "LLI" => Ok(Self::LLI),
            "LUI" => Ok(Self::LUI),
            "BNEZ" => Ok(Self::BNEZ),
            "BEQZ" => Ok(Self::BEQZ),
            "BMI" => Ok(Self::BMI),
            "BPL" => Ok(Self::BPL),
            "JMP" => Ok(Self::JMP),
            "SLB" => Ok(Self::SLB),
            "SRB" => Ok(Self::SRB),
            "SRHB" => Ok(Self::SRHB),
            "LL" => Ok(Self::LL),                        
            _ => Err(AsmError::new("invalid token")),
        }
    }
}

impl Mnemonic {
    pub fn into_u16(&self) -> u16 {
        match self {
            Self::NOP => 0b00000_000_000_00000,
            Self::MV => 0b00000_000_000_00001,
            Self::NOT => 0b00000_000_000_00010,
            Self::XOR => 0b00000_000_000_00011,
            Self::ADD => 0b00000_000_000_00100,
            Self::SUB => 0b00000_000_000_00101,
            Self::SLB => 0b00000_000_000_00110,
            Self::SRB => 0b00000_000_000_00111,
            Self::SL => 0b00000_000_000_01000,
            Self::SR => 0b00000_000_000_01001,
            Self::AND => 0b00000_000_000_01010,
            Self::OR => 0b00000_000_000_01011,
            Self::SRHB => 0b00000_000_000_01100,
            Self::LL => 0b00000_000_000_01101,
            Self::ST => 0b00000_000_000_10000,
            Self::SBU => 0b00000_000_000_10010,
            Self::LD => 0b00000_000_000_10001,
            Self::LBU => 0b00000_000_000_10011,
            Self::ADDI => 0b00100_000_00000000,
            Self::ANDI => 0b01010_000_00000000,
            Self::ORI => 0b01011_000_00000000,
            Self::LLI => 0b00001_000_00000000,
            Self::LUI => 0b00110_000_00000000,
            Self::BNEZ => 0b10000_000_00000000,
            Self::BEQZ => 0b10001_000_00000000,
            Self::BMI => 0b10010_000_00000000,
            Self::BPL => 0b10011_000_00000000,
            Self::JMP => 0b11000_00000000000,
        }
    }
}

#[derive(Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

impl FromStr for Register {
    type Err = AsmError;
    fn from_str(s: &str) -> Result<Self> {
        let tmp = String::from(s).to_ascii_uppercase();
        match tmp.as_ref() {
            "R0" => Ok(Self::R0),
            "R1" => Ok(Self::R1),
            "R2" => Ok(Self::R2),
            "R3" => Ok(Self::R3),
            "R4" => Ok(Self::R4),
            "R5" => Ok(Self::R5),
            "R6" => Ok(Self::R6),
            "R7" => Ok(Self::R7),
            _ => Err(AsmError::new("invalid register")),
        }
    }
}

impl Register {
    pub fn reg2bin(&self) -> u16 {
        match self {
            Self::R0 => 0,
            Self::R1 => 1,
            Self::R2 => 2,
            Self::R3 => 3,
            Self::R4 => 4,
            Self::R5 => 5,
            Self::R6 => 6,
            Self::R7 => 7,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    mnemonic: Mnemonic,
    operand1: Option<u16>,
    operand2: Option<u16>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = self.into_u16();
        write!(f, "{:0>8b} {:0>8b}", code >> 8, code & 0xFF)
    }
}

use Mnemonic::*;
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
