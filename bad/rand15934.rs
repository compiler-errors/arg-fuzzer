
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15934(_: S2, _: S6, _: S7, _: S0) {}
        
        fn test15934() { foo15934(S1, S4, S6, S7, S8); }
    