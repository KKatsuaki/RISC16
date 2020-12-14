use std::fmt;
use regex;

#[derive(Debug)]
pub struct AsmError{}
impl fmt::Display for AsmError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{:?}", self)	
    }
}
impl std::error::Error for AsmError{}


#[derive(Debug)]
pub enum Mnemonic{
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
    DATA(u16)
}

use std::str::FromStr;
impl FromStr for Mnemonic{
    type Err = AsmError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let tmp = String::from(s);

	tmp.to_ascii_uppercase();
	match tmp.as_ref(){
	    "NOP" => Ok(NOP),
	    "MV"  => Ok(MV), 
	    "NOT" => Ok(NOT),
	    "XOR" => Ok(XOR),
	    "ADD" => Ok(ADD),
	    "SUB" => Ok(SUB),
	    "SL"  => Ok(SL), 
	    "SR"  => Ok(SR), 
	    "AND" => Ok(AND),
	    "OR"  => Ok(OR), 
	    "ST"  => Ok(ST), 
	    "LD"  => Ok(LD), 
	    "ADDI" => Ok(ADDI),
	    "ANDI" => Ok(ANDI),
	    "ORI" => Ok(ORI),
	    "LLI" => Ok(LLI),
	    "LUI" => Ok(LUI),
	    "BNEZ" => Ok(BNEZ),
	    "BEQZ" => Ok(BEQZ),
	    "BMI" => Ok(BMI),
	    "BPL" => Ok(BPL),
	    "JMP" => Ok(JMP),
	    _ => {
		match handle_operand(&tmp,16){
		    Some(b) => Ok(DATA(b)),
		    None => Err(AsmError{})
		}
	    }
	}
    }
}


use Mnemonic::*;
impl Mnemonic{
    pub fn into_u16(&self) -> u16{
	match self{
	    NOP  => 0b00000_000_000_00000,
	    MV   => 0b00000_000_000_00001,
	    NOT  => 0b00000_000_000_00010,
	    XOR  => 0b00000_000_000_00011,
	    ADD  => 0b00000_000_000_00100,
	    SUB  => 0b00000_000_000_00101,
	    SL   => 0b00000_000_000_01000,
	    SR   => 0b00000_000_000_01001,
	    AND  => 0b00000_000_000_01010,
	    OR   => 0b00000_000_000_01011,
	    ST   => 0b00000_000_000_10000,
	    LD   => 0b00000_000_000_10001,
	    ADDI => 0b00100_000_00000000,
	    ANDI => 0b01010_000_00000000,
	    ORI  => 0b01011_000_00000000,
	    LLI  => 0b00001_000_00000000,
	    LUI  => 0b00110_000_00000000,
	    BNEZ => 0b10000_000_00000000,
	    BEQZ => 0b10001_000_00000000,
	    BMI  => 0b10010_000_00000000,
	    BPL  => 0b10011_000_00000000,
	    JMP  => 0b11000_00000000000,
	    DATA(d) => *d,
	}                 
    }
}

#[derive(Debug)]
pub struct Instruction{
    mnemonic : Mnemonic,
    operand1 : Option::<u16>,
    operand2 : Option::<u16>,
}

impl fmt::Display for Instruction{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let op1 = match self.operand1{
	    Some(s) => s,	    
	    None => 0,
	};
	let op2 = match self.operand2{
	    Some(s) => s,	    
	    None => 0,
	};
	write!(f, "{:?} {} {}",self.mnemonic,op1,op2)
    }
}


impl Instruction{
    pub fn new(mnemonic : Mnemonic, op1 : Option::<u16>, op2 : Option::<u16>) -> Self{
	Self{
	    mnemonic,
	    operand1 : op1,
	    operand2 : op2
	}
    }

    pub fn in_ascii(&self) -> String{
	format!("{:0>8b} {:0>8b}", self.into_u16() >> 8,self.into_u16() & 0b11111111)
    }

    pub fn asm2inst(line : &str) -> Self{
	let line = String::from(line);
	let mut tokens : Vec::<_> = line.split(|ch:char| ch == ',' || ch.is_whitespace()).collect();
	tokens.retain(|s| s!=&"");

	let mut tokens = tokens.iter();

	let op = match tokens.next(){
	    Some(s) => match Mnemonic::from_str(&s){
		Ok(b) => b,
		Err(e) => {
		    println!("{} isn't reserved", s);
		    panic!("{}",e.to_string());
		},
	    }
	    None => {
		panic!("nothing");
	    }
	};

	let bits = match op{
	    JMP => 11,
	    _ => 8
	};

	let op1 = match tokens.next(){
	    Some(s) => handle_operand(s,bits),
	    None => None
	};
	
	let op2 = match tokens.next(){
            Some(s) => handle_operand(s,bits),
	    None => None              
	};

	Self{mnemonic : op, operand1:op1, operand2:op2}
    }

    pub fn into_u16(&self) -> u16{
	let ddd = match self.operand1{
	    Some(b) => b,
	    None => 0
	};

	let sss = match self.operand2{
            Some(b) => b,             
            None => 0                 
	};

	let op = self.mnemonic.into_u16();
	
	match &self.mnemonic{
	    // Jump Type
	    JMP => {
		/* bitwise operation*/
		op | ddd
	    },

	    // Branch Type and Immediate Type
	    BEQZ | BNEZ | BMI | BPL | ADDI| ANDI| ORI| LLI| LUI => {
		/* bitwise operation*/
		op | ddd << 8 | sss
	    },

	    // Memory Type and Register Type
	    _ => {
		/* bitwise operation*/
		ddd << 8 | sss << 5 | op
	    },
	}	
    }
}

pub fn reg2bin(reg : &str) -> Option::<u16>{
    let mut reg = String::from(reg);
    reg = reg.to_ascii_uppercase();
    let reg = match reg.as_ref(){ 
	"R0" => 0b0,
	"R1" => 0b1,
	"R2" => 0b10,
	"R3" => 0b11,
	"R4" => 0b100,
	"R5" => 0b101,
	"R6" => 0b110,
	"R7" => 0b111,
	_ => 0b1111,
    };
    if reg > 7{
	None
    }else{
	Some(reg)
    }
}

// convert negative number to 2's complement
pub fn conv_complement(n : i16, bits : usize) -> u16{
    let n = if n < 0 {(n * (-1)) as u16} else {n as u16};
    (!n+1) & (0b1111_1111_1111_1111) >> (16-bits)
}


fn handle_operand(operand : &str, bits : usize) -> Option::<u16>{
    let re_imm_dec = regex::Regex::new(r"#(\-?[0-9_]+)").unwrap();
    let re_imm_hex = regex::Regex::new("#0x([0-9a-fA-F_]+)").unwrap();
    let re_reg = regex::Regex::new(r"\(?([rR][0-8_])\)?").unwrap();
    let re_imm_bin = regex::Regex::new("#0b([0-1_]+)").unwrap();
    let mut op = 0;
    let mut res = false;

    let mut operand = String::from(operand);
    operand.retain(|ch| !ch.is_whitespace());    

    match re_imm_dec.captures(&operand){
	Some(b) => {
	    let mut temp = String::from_str(b.get(1).unwrap().as_str()).unwrap();
	    temp.retain(|ch| ch != '_');
	    let tmp : i16 = temp.parse().unwrap();
	    op = if tmp < 0 {conv_complement(tmp, bits)}else{tmp as u16};
	    res |= true;
	},
	None => (),
    }

    match re_imm_hex.captures(&operand){
	Some(b) => {
            let mut temp = String::from_str(b.get(1).unwrap().as_str()).unwrap();
            temp.retain(|ch| ch != '_');                                         
	    op = u16::from_str_radix(&temp,16).unwrap();
	    res |= true;
	},
	None => (),
    }

    match re_imm_bin.captures(&operand){
        Some(b) => {                                  
            let mut temp = String::from_str(b.get(1).unwrap().as_str()).unwrap();
            temp.retain(|ch| ch != '_');
            op = u16::from_str_radix(&temp,2).unwrap();
	    res |= true;
        },                                            
        None => (),                                   
    }
    
    match re_reg.captures(&operand){
        Some(b) => {
	    let tmp = b.get(1).unwrap().as_str();
	    op = match reg2bin(tmp){
		Some(s) => s,
		None => 0,
	    };
	    res |= true;
	},
        None => (),                           
    }                                           
    if res {Some(op)}else{None}
}
