
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15829(_: S2, _: S4, _: S5, _: S7) {}
        
        fn test15829() { foo15829(S1, S6, S3, S5, S6, S5, S0); }
    