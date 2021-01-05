        LLI r3, #0x2e
        LLI r4, #0x30
        LLI r5, #0x32
        LUI r6, #0x02
        LD r0, (r3)
        LD r1, (r4)
        LD r2, (r5)
        NOP
        NOP
        ADD r2, r0
        ADDI r1, #-1
        NOP
        NOP
        BNEZ r1, #-10
        NOP
        NOP
        NOP
        ST r2, (r5)
        ST r2, (r6)
        JMP #-2
        NOP
        NOP
        NOP
        #0b0001001000110100
        #0b0000000000000100
        #0b0000000000000000
