
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9738(_: S3, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test9738() { foo9738(S1, S7, S0, S6, S5, S5, S7); }
    