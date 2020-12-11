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
**Register Type**
| Instruction Code | Mnemonic | Description  | behavior      |
|------------------|----------|--------------|-------------|
| 00000000000 00000 | NOP      | No Operation | r0 <- r0    |
| 00000dddsss 00001 | MV d,s   | Move         | d <- s      |
| 00000dddsss 00010 | NOT d,s  | Not          | d <- ~s     |
| 00000dddsss 00011 | XOR d,s  | Exclusive OR | d <- d ^ s  |
| 00000dddsss 00100 | ADD d,s  | Add          | d <- d + s  |
| 00000dddsss 00101 | SUB d,s  | Subtruct     | d <- d - s  |
| 00000dddsss 01000 | SL d,s   | Shift Left   | d <- s << 1 |
| 00000dddsss 01001 | SR d,s   | Shift Right  | d <- s >> 1 |
| 00000dddsss 01010 | AND d,s  | AND          | d <- d & s  |
| 00000dddsss 01011 | OR d,s   | OR           | d <- d \| s  |

**Memory Type**
| Instruction Code | Mnemonic  | Description | behavior                                                       |
|------------------|-----------|-------------|----------------------------------------------------------------|
| 00000dddsss10000 | ST d, (s) | Store       | Store the content of the register ddd where indicated by sss   |
| 00000dddsss10001 | LD d, (s) | Load        | Load the content on the address which is indicated by sss to d |

**Immediate Type**
| Instruction Code | Mnemonic  | Description          | behavior                                           |
|------------------|-----------|----------------------|----------------------------------------------------|
| 00100dddxxxxxxxx | ADDI d,#X | Add Immediate        | d <- d + X                                         |
| 01010dddxxxxxxxx | ANDI d,#X | And Immediate        | d <- d & X                                         |
| 01011dddxxxxxxxx | ORI d,#X  | Or Immediate         | d <- d \| X                                        |
| 00001dddxxxxxxxx | LLI d,#X  | Load Lower Immediate | store the value x on register d                    |
| 00110dddxxxxxxxx | LUI d,#X  | Load Upper Immediate | store the value x on the upper 8bits of register d |

**Branch Type**
| Instruction Code | Mnemonic | Description              | behavior                              |
|------------------|----------|--------------------------|---------------------------------------|
| 10000dddxxxxxxxx | BNEZ d,X | Branch on Not Equal Zero | if register d isn't 0, add X to PC    |
| 10001dddxxxxxxxx | BEQZ d,X | Branch on Equal Zero     | if register d is 0, add X to PC       |
| 10010dddxxxxxxxx | BMI d,X  | Branch on Minus          | if register d is negative add X to PC |
| 10011dddxxxxxxxx | BPL d,X  | Branch on Plus           | if register d is positive add X to PC |

PC: Program counter

**Jump Type**
| Instruction Code | Mnemonic | Description | behavior                       |
|------------------|----------|-------------|--------------------------------|
| 11000xxxxxxxxxxx | JMP X    | Jump        | Without condition, add X to PC |


## Asm
An assembler for this RISC16 arch written in Rust. This assmbler generates binary in ascii text.

`asm <input>` or `asm <input> <output>`

If you don't specify the output file, this program will generate sim_risc16.mem.

The Makefile in risc16 directory is only support sim_risc16.mem for simulation.

`make sim` is the command to execute the simulation.
