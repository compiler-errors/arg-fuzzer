
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3483(_: S6, _: S7, _: S1, _: S5) {}
        
        fn test3483() { foo3483(S0, S3, S2, S7, S4, S0); }
    