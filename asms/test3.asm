	LUI r6, #0x2
	LUI r7, #0x1
	ORI r7, #0x10
	LUI r5, #0x1
	LD r0, (r5)
	ADDI r5, #2
	MV r4, r7
	SUB r4, r5
	BEQZ r4, #12
	LD r2, (r5)
	MV r1, r2
	SUB r1, r0
	BPL r1, #2
	MV r0, r2
	JMP #-20
	ST r0, (r6)
	JMP #-2
	@100 #0xff
	@102 #0x14
	@104 #0x5ff
	@106 #0xab
	@108 #0x111
	@10a #0x123
	@10c #0x1041
	@10e #0x12        
