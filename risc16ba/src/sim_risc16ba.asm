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
        BNEZ r5, READ_IMAGE // end of reading image
        LUI r0, #0x7f
        LUI r1, #0xff
        LLI r2, #0x80
WRITE_OUTER:
        LD r7, (r0)
        SR r7, r7
        SR r7, r7
        ADDI r0, #2
        LLI r3, #0x80
        MV r5, r1
        OR r5, r2
        LLI r6, #0xff
LOOP1:  
        BEQZ r7, LOOP1E
        SBU r6, (r5)        
        ADDI r7, #-1
        ADDI r3, #-1
        ADDI r5, #-128
        JMP LOOP1
LOOP1E: 
        LLI r6, #0x0	
LOOP2:    
        BEQZ r3, LOOP2E
	SBU r6, (r5)    
        ADDI r3, #-1  	
        ADDI r5, #-128
        JMP LOOP2
LOOP2E:
        ADDI r2, #1
        MV r7, r2
        ANDI r7, #0xff
        BNEZ r7, WRITE_OUTER
        JMP #-2
        
