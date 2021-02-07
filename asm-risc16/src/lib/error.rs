use std::fmt;
#[derive(Debug)]
pub struct AsmError {}
impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AsmError {}

impl AsmError {
    pub fn new() -> Self {
        Self {}
    }

    pub fn boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}
