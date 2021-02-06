pub mod risc16{
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
        SBU,
        LBU,
        DATA(u16),
        DataWithAddr(u16)
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
                "LBU" => Ok(LBU),
                "SBU" => Ok(SBU),                
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
                    let mut tmp = tmp.clone();
                    tmp.retain(|ch| ch != '_');
                    let re_addr = regex::Regex::new(r"@(0x)?([0-9a-fA-F]+)").unwrap();

                    match re_addr.captures(&tmp){
                        Some(cap) => {
                            let temp = String::from_str(cap.get(2).unwrap().as_str()).unwrap();
                            let addr = u16::from_str_radix(&temp,16).unwrap();
                            Ok(DataWithAddr(addr))
                        }

                        None => {
                            let d = handle_operand(&tmp, 16);
                            match d{
                                Some(d) => Ok(DATA(d)),
                                None => Err(AsmError{})
                            }
                        }
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
                SBU  => 0b00000_000_000_10010,
                LD   => 0b00000_000_000_10001,
                LBU  => 0b00000_000_000_10011,
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
                DataWithAddr(_) => 0,
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
            write!(f, "{:?} 0x{:x} 0x{:x}",self.mnemonic,op1,op2)
        }
    }


    impl Instruction{
        pub fn new(mnemonic : Mnemonic, operand1 : Option::<u16>, operand2 : Option::<u16>) -> Self{
            Self{
                mnemonic,
                operand1,
                operand2,
            }
        }

        pub fn in_ascii(&self, offset : &mut u16) -> String{
            match self.mnemonic{
                DataWithAddr(addr) => format!("@{:x} {:0>8b} {:0>8b}", addr , self.into_u16() >> 8, self.into_u16() & 0b1111_1111),
                DATA(d) => {
                    let res = format!("@{:x} {:0>8b} {:0>8b}",offset ,d >> 8,d & 0b1111_1111);
                    *offset += 2;
                    res
                }
                _ => {
                    let res = format!("@{:x} {:0>8b} {:0>8b}",offset ,self.into_u16() >> 8,self.into_u16() & 0b1111_1111);
                    *offset += 2;
                    res
                }
            }
        }

        pub fn asm2inst(line : &str) -> Self{
            // buffer 
            let line = String::from(line);

            // split buffer at `,` or some whitespaces and remove null string from the splited strings vector
            let mut tokens : Vec::<_> = line.split(|ch:char| ch == ',' || ch.is_whitespace()).collect();
            tokens.retain(|s| s !=& "");

            let mut tokens = tokens.iter();

            // get Mnemonic
            let op = match tokens.next(){
                Some(s) => match Mnemonic::from_str(&s){
                    Ok(b) => b,
                    Err(e) => {
                        println!("{} isn't reserved", s);
                        panic!("{}",e.to_string());
                    },
                }
                None => {
                    panic!("paniced");
                }
            };

            // getting the bit width of operand
            let bits = match op{
                JMP => 11,
                DataWithAddr(_) => 16,
                DATA(_) => 16,
                _ => 8
            };
            
            // handle operand
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

                DataWithAddr(_) => {
                    op | ddd | sss
                }

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
        match reg.as_ref(){ 
            "R0" => Some(0b0),
            "R1" => Some(0b1),
            "R2" => Some(0b10),
            "R3" => Some(0b11),
            "R4" => Some(0b100),
            "R5" => Some(0b101),
            "R6" => Some(0b110),
            "R7" => Some(0b111),
            _ => None
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
}

pub mod env{
    use std::path::*;                                                   
                                                                        
    #[derive(Debug)]                                                    
    pub struct Config{                                                  
        out_path : Option::<PathBuf>,                                   
        input_path : Option::<PathBuf>,                                 
        on_stdout : bool                                                
    }                                                                   
                                                                        
    impl Config{                                                        
        pub fn new() -> Self{                                           
            let mut args = std::env::args();                            
                                                                        
            // init                                                     
            let mut out : Option::<PathBuf> = None;                     
            let mut input : Option::<PathBuf> = None;                   
            let mut on_stdout : bool = false;                           
                                                                        
            // handle arguments                                         
            loop{                                                       
                match args.next(){                                      
                    Some(s) => {                                        
                        match s.as_str(){                               
                            "-o" => match args.next() {                 
                                Some(b) => out = Some(PathBuf::from(b)),
                                None => ()                              
                            },                                          
                                                                        
                            "-stdout" | "--stdout"=> on_stdout = true,  
                            _ => input = Some(PathBuf::from(s)),        
                        }                                               
                    }                                                   
                    None => break,                                      
                }                                                       
            }                                                           
                                                                        
            Self{                                                       
                out_path : out,                                         
                input_path : input,                                     
                on_stdout                                               
            }                                                           
        }                                                               
                                                                        
        pub fn get_out_path(&self) -> PathBuf{                          
            match &self.out_path{                                       
                Some(p) => p.clone(),                                   
                None => PathBuf::from(String::from("sim_risc16.mem"))   
            }                                                           
        }                                                               
                                                                        
        pub fn get_input_path(&self) -> PathBuf{                        
            match &self.input_path{                                     
                Some(p) => p.clone(),                                   
                None => panic!("No input file")                         
            }                                                           
        }                                                               
                                                                        
        pub fn is_on_stdout(&self) ->  bool{                            
            self.on_stdout                                              
        }                                                               
    }                                                                   
}                                                                       
