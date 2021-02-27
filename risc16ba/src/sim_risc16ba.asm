	LUI r0, #0x7f
	LUI r1, #0x80
	LLI r2, #0x0
INIT_ARRAY:	
        ST  r2, (r0)
        ADDI r0, #2
        MV r3, r1
        SUB r3, r0
        BNEZ r3, INIT_ARRAY
        LUI r1, #0x7f 
        LUI r2, #0xc0
        LLI r7, #0x0
        LL r6, r0
        ADD r6, r2        
READ_IMAGE:
	LD r3, (r0)
        SRB r5, r3
        LL r3, r3
        ADD r3, r1        
	LD  r4, (r3)    
	ADDI r4, #1
        ST r7, (r6)
	ST  r4, (r3)    	
        ADDI r6, #2
        ADDI r0, #2
        ADD r5, r1
        LD r4, (r5)
	ADDI r4, #1        
	ST  r4, (r5)
        MV r5, r2
        SUB r5, r0
        BNEZ r5, READ_IMAGE
        LUI r0, #0x7f
        LUI r1, #0xff
        LLI r2, #0x80
WRITE_OUTER:
        LD r7, (r0)
        SRHB r7, r7
        ADDI r0, #2
        MV r5, r1
        OR r5, r2
        LLI r6, #0xff
LOOP1:  
        BEQZ r7, LOOP1E
        SBU r6, (r5)        
        ADDI r7, #-1
        ADDI r5, #-128        			
        JMP LOOP1
LOOP1E: 
        ADDI r2, #1
        MV r7, r2
        ANDI r7, #0xff
        BNEZ r7, WRITE_OUTER
        JMP #-2
        
