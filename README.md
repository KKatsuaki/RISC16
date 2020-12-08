# RISC16
I made this repo for university's class. I hope this helps your learning.

## RISC16
### Instruction set
#### formats
|                | 15 | 14      | 13      | 12      | 11      | 10    | 9   | 8   | 7    | 6    | 5    | 4    | 3       | 2       | 1       | 0       |
|----------------|----|---------|---------|---------|---------|-------|-----|-----|------|------|------|------|---------|---------|---------|---------|
| Register type  | 0  | 0       | 0       | 0       | 0       | Rd2   | Rd1 | Rd0 | Rs2  | Rs1  | Rs0  | 0    | ALU_OP3 | ALU_OP2 | ALU_OP1 | ALU_OP0 |
| Memory type    | 0  | 0       | 0       | 0       | 0       | Rd2   | Rd1 | Rd0 | Rs2  | Rs1  | Rs0  | 1    | func3   | func2   | func1   | func0   |
| Immediate type | 0  | ALU_OP3 | ALU_OP2 | ALU_OP1 | ALU_OP0 | Rd2   | Rd1 | Rd0 | IMM7 | IMM6 | IMM5 | IMM4 | IMM3    | IMM2    | IMM1    | IMM0    |
| Branch type    | 1  | 0       | cond2   | cond2   | cond1   | Rd2   | Rd1 | Rd0 | OS7  | OS6  | OS5  | OS4  | OS3     | OS2     | OS1     | OS0     |
| Jump type      | 1  | 1       | func3   | func2   | func1   | func0 | OS9 | OS8 | OS7  | OS6  | OS5  | OS4  | OS3     | OS2     | OS1     | OS0     |


OS: offset.

IMM: Immediate operand.

Rd: register(destination).

Rs: register(source).

#### opcodes
Arithmetic Mnemonics
| Mnemonic | code(binary) | description |
|----------|--------------|-------------|
| ADD      | 00100        | Addtion     |
| SUB      | 00101        | subdract    |


## Asm
An assembler written in Rust

### Usage

