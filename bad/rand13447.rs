
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13447(_: S7, _: S8, _: S1, _: S4) {}
        
        fn test13447() { foo13447(S6, S2, S7, S4, S0, S7); }
    