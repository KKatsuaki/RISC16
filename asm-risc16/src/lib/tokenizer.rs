use crate::lib::error::AsmError;
use crate::lib::result::Result;
use crate::lib::risc16::{Mnemonic, Register};
use crate::regex::Regex;

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    Comment,
    Mnemo(Mnemonic),
    SetLabel(String),
    Label(String),
    Reg(Register),
    Data(i16),
    Addr(u16),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            Self::Data(d) => format!("{:x}", d),
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
        let re_label = Regex::new(r"^(?P<label>[a-zA-Z][a-zA-Z0-9_]*)").unwrap();
        let re_register = Regex::new(r"^\(?(?P<reg>[rR][0-7])\)?").unwrap();
        let re_setlabel = Regex::new(r"^(?P<label>[a-zA-Z][a-zA-Z0-9_]*):").unwrap();
        let re_data = Regex::new(
            r"\#(?P<val>0x(?P<hex>[0-9a-fA-F_]+)|0b(?P<bin>[0-1_]+)|(?P<dec>\-?([0-9_]+)))",
        )
        .unwrap();
        let re_addr = Regex::new(r"(@(?P<addr>[0-9A-Fa-f]+))").unwrap();

        // removing parentheses
        let mut _tok = tok.replace(",", "");
        let tok = _tok.as_str();

        //println!("tok: {}", tok);

        if re_comment.is_match(tok) {
            // check if comment
            Ok(Self::Comment)
        } else {
            match re_addr.captures(tok) {
                // addr
                Some(cap1) => match u16::from_str_radix(cap1.name("addr").unwrap().as_str(), 16) {
                    Ok(b) => Ok(Self::Addr(b)),
                    Err(_) => Err(AsmError::new("invalid addr")),
                },
                None => match Mnemonic::from_str(tok) {
                    // convert Mnenimonic
                    Ok(b) => Ok(Self::Mnemo(b)),
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
                                                    Err(_) => {
                                                        return Err(AsmError::new("invalid hex"))
                                                    }
                                                }
                                            }
                                            None => match cap4.name("bin") {
                                                Some(bin_val) => {
                                                    match i16::from_str_radix(bin_val.as_str(), 2) {
                                                        Ok(v) => v,
                                                        Err(_) => {
                                                            return Err(AsmError::new(
                                                                "invalid binary",
                                                            ))
                                                        }
                                                    }
                                                }

                                                None => match cap4.name("dec") {
                                                    Some(dec_val) => {
                                                        match dec_val.as_str().parse::<i16>() {
                                                            Ok(v) => v,
                                                            Err(_) => {
                                                                return Err(AsmError::new(
                                                                    "invalid integer",
                                                                ))
                                                            }
                                                        }
                                                    }
                                                    None => {
                                                        return Err(AsmError::new("invalid token"))
                                                    }
                                                },
                                            },
                                        };
                                        Ok(Self::Data(val))
                                    }
                                    None => return Err(AsmError::new("invalid token")),
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
        for (i, tok) in buf.split_whitespace().enumerate() {
            let tmp = Self::parse(tok)?;
            match tmp {
                Self::SetLabel(ref tmp) => {
                    let cur_addr = if *linum == 0 { 0 } else { (*linum - 1) * 2 };
                    label_map.insert(tmp.clone(), cur_addr);
                    *linum -= 1;
                }
                Self::Comment => {
                    if i == 0 {
                        *linum -= 1;
                    };
                    break;
                }
                _ => (),
            }
            res.push(tmp);
        }
        *linum += 1;
        Ok(res)
    }
}
