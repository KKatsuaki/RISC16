	LUI r0, #0x7f
	LUI r1, #0x80
	LLI r2, #0x0
INIT_ARRAY:	
        ST  r2, (r0)
        ADDI r0, #2
        MV r3, r1
        SUB r3, r0
        BNEZ r3, INIT_ARRAY // end of initilize array
        LUI r1, #0x7f 
        // r0 : head of image, r1 : head of array, r2 : head of histgram
        LUI r2, #0xc0
READ_IMAGE:
        LBU r3, (r0)
        ADD r3, r1
        LD  r4, (r3)
        ADDI r4, #1
        ST  r4, (r3)
        ADDI r0, #1
        MV r5, r2
        SUB r5, r0
        BNEZ r5, READ_IMAGE        
        JMP #-2
        
