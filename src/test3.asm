	LUI r6, #0x2
	LUI r7, #0x1
	ORI r7, #0x10
	LUI r5, #0x1
	LD r0, (r5)
	ADDI r5, #2
	MV r4, r7
	SUB r4, r5
	BEQZ r4, #30
	NOP
	NOP
	NOP                   
	LD r2, (r5)
	MV r1, r2
	SUB r1, r0
	BPL r1, #8
        NOP
        NOP
        NOP
	MV r0, r2
	JMP #-32
        NOP
        NOP
        NOP
	ST r0, (r6)
	JMP #-2
        NOP
        NOP
        NOP
        @10f #0x12
        @100 #0x14
        @102 #0x5ff
        @104 #0xab
        @106 #0x111
        @108 #0x123
        @10a #0x1041
        @10c #0xff