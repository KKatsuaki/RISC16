use std::fmt;
use std::str::FromStr;
#[derive(Debug)]
pub struct AsmError {
    msg: String,
}
impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for AsmError {}

impl AsmError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: String::from_str(msg).unwrap(),
        }
    }

    pub fn boxed(msg: &str) -> Box<Self> {
        Box::new(Self::new(msg))
    }
}
