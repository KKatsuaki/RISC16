        LUI r7, #0x7F
        ORI r7, #0x80
        LUI r6, #0x80
        LUB r0, (r6)
        SR r1, r0
        ADD r1, r7
        
