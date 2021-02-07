use crate::lib::error::AsmError;
use crate::lib::result::Result;
use crate::lib::risc16::{Mnemonic, Register};
use crate::regex::Regex;

use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;
/*
data format
dec : r"#(\-?[0-9_]+)"
hex : r"#0x([0-9a-fA-F_]+)"
bin : r"#0b([0-1_]+)"
 */
#[derive(Debug)]
pub enum Token {
    Comment,          // r"//[ \t.]*"
    Mnemo(Mnemonic),  // one of menimonic such as NOP, MV, etc
    SetLabel(String), // r"^[ \t]*(?P<label>[a-zA-Z][a-zA-Z0-9]*):"
    Label(String),    // r"(?P<label>[a-zA-Z][a-zA-Z0-9]*)"
    Reg(Register),    // r"(?P<reg>[rR][0-7])"
    Data(i16), // r"#(?P<val>0x(?P<hex>[0-9a-fA-F_]+)|0b(?P<bin>[0-1_]+)|(?P<dec>\-?([0-9_]+)))"
    Addr(u16), // r"(@(?P<addr>[0-9A-Fa-f]*))
}

impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self{
            Self::Data(d) => format!("{}", d),
            Self::Label(lab) => format!("{}", lab),
            Self::Reg(r) => format!("{:?}", r),
            Self::Addr(addr) => format!("@{:0>4x}", addr),
            Self::Mnemo(mnemonic) => format!("{:?}", mnemonic),
            Self::Comment => format!(""),
            Self::SetLabel(lab) => format!("{}", lab),
        };
        write!(f, "{}", res)
    }
}

impl Token {
    pub fn parse(tok: &str) -> Result<Self> {
        let re_comment = Regex::new(r"//[ .\t]*").unwrap();
        let re_label = Regex::new(r"^(?P<label>[a-zA-Z][a-zA-Z0-9]*)").unwrap();
        let re_register = Regex::new(r"(?P<reg>[rR][0-7])").unwrap();
        let re_setlabel = Regex::new(r"^[ \t]*(?P<label>[a-zA-Z][a-zA-Z0-9]*):").unwrap();
        let re_data = Regex::new(
            r"#(?P<val>0x(?P<hex>[0-9a-fA-F_]+)|0b(?P<bin>[0-1_]+)|(?P<dec>\-?([0-9_]+)))",
        )
        .unwrap();
        let re_addr = Regex::new(r"(@(?P<addr>[0-9A-Fa-f]+))").unwrap();

        if re_comment.is_match(tok) {
            // check if comment
            Ok(Self::Comment)
        } else {
            match re_addr.captures(tok) {
                // addr
                Some(cap1) => match u16::from_str_radix(cap1.name("addr").unwrap().as_str(), 16) {
                    Ok(b) => Ok(Self::Addr(b)),
                    Err(_) => Err(AsmError::new()),
                },
                None => match Mnemonic::from_str(tok) {
                    Ok(b) => Ok(Self::Mnemo(b)), // convert Mnenimonic
                    Err(_) => match re_register.captures(tok) {
                        // reg
                        Some(cap2) => Ok(Self::Reg(Register::from_str(
                            cap2.name("reg").unwrap().as_str(),
                        )?)),
                        None => match re_setlabel.captures(tok) {
                            Some(cap5) => Ok(Self::SetLabel(
                                cap5.name("label").unwrap().as_str().to_string(),
                            )),
                            None => match re_label.captures(tok) {
                                // label
                                Some(cap3) => Ok(Self::Label(
                                    cap3.name("label").unwrap().as_str().to_string(),
                                )),
                                None => match re_data.captures(tok) {
                                    // data
                                    Some(cap4) => {
                                        let val = match cap4.name("hex") {
                                            Some(hex_val) => {
                                                match i16::from_str_radix(hex_val.as_str(), 16) {
                                                    Ok(v) => v,
                                                    Err(_) => return Err(AsmError::new()),
                                                }
                                            }
                                            None => match cap4.name("bin") {
                                                Some(bin_val) => {
                                                    match i16::from_str_radix(bin_val.as_str(), 2) {
                                                        Ok(v) => v,
                                                        Err(_) => return Err(AsmError::new()),
                                                    }
                                                }
                                                None => match cap4.name("dec") {
                                                    Some(dec_val) => {
                                                        match dec_val.as_str().parse::<i16>() {
                                                            Ok(v) => v,
                                                            Err(_) => return Err(AsmError::new()),
                                                        }
                                                    }
                                                    None => return Err(AsmError::new()),
                                                },
                                            },
                                        };
                                        Ok(Self::Data(val))
                                    }
                                    None => Err(AsmError::new()),
                                },
                            },
                        },
                    },
                },
            }
        }
    }

    pub fn tokenize(
        l: &str,
        linum: &mut u16,
        label_map: &mut HashMap<String, u16>,
    ) -> Result<Vec<Self>> {
        let buf = String::from_str(l).unwrap();
        let mut res = Vec::new();
        for tok in buf.split_whitespace() {
            let tmp = Self::parse(tok)?;
            match tmp {
                Self::SetLabel(ref tmp) => {
                    let cur_addr = if *linum == 0{
                        0
                    }else{
                        (*linum - 1) * 2
                    };
                    label_map.insert(tmp.clone(), cur_addr);
                    *linum -= 1;
                }
                _ => (),
            }
            res.push(tmp);
        }
        *linum += 1;
        Ok(res)
    }
}
