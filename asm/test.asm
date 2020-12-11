	LLI r3, #0x20
	LD r0, (r3)
	LLI r4, #0x22
	LD r1, (r4)
	LLI r5, #0x24
	LD r2, (r5)
	ADD r2, r0
	ADDI r1, #-1
	BNEZ r1, #-6
	ST r2,(r5)
	LUI r6,(r5)
	ST r2,(r6)
	JMP #-2
	
