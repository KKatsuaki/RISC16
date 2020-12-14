	LLI r0, #0x34
	LUI r0, #0x12
	LLI r4, #0x22
	LLI r1, #0x4
	LLI r5, #0x24
	LD r2, (r5)
	ADD r2, r0
	ADDI r1, #-1
	BNEZ r1, #-6
	ST r2,(r5)
	LUI r6,#0x02
	ST r2,(r6)
	JMP #-2
	NOP
	NOP
	#0b0001_0010_0011_0100
	#0b0000_0000_0000_0100
	NOP
