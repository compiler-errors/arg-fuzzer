
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15371(_: S7, _: S2, _: S0, _: S3) {}
        
        fn test15371() { foo15371(S6, S6, S5, S0, S1, S7, S0); }
    