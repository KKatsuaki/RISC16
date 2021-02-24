	LUI r0,  #0x40
	LUI r1,  #0x80
	LUI r2,  #0xc0
LOOP:	
	LBU r3,  (r1)
	LLI r4,  #0xff
	SUB r4,  r3
	SBU r4,  (r2)
	ADDI r1, #1
	ADDI r0, #-1
	ADDI r2, #1
	BNEZ r0, LOOP
	JMP  #-2
