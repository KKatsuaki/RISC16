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
}

pub struct Instruction{
    mnemonic : Mnemonic,
    operand1 : u16,
    operand2 : Option::<u16>,
}

