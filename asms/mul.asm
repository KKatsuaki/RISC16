	LLI r3, #0x20
	LD r0, (r3)
	LLI r4, #0x22
	LD r1, (r4)
	LLI r5, #0x24
	LD r2, (r5)
	ADD r2, r0
	ADDI r1, #-1
	BNEZ r1, #-6
	ST r2, (r5)
	LUI r6, #0x02
	ST r2, (r6)
	JMP #-2
	@20 #0x1234
	@22 #0x4
	@24 #0x0
