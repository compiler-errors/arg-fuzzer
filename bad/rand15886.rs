
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15886(_: S0, _: S7, _: S3, _: S0, _: S1, _: S5) {}
        
        fn test15886() { foo15886(S1, S6, S7, S3, S2, S1, S6); }
    