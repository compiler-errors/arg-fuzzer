
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14115(_: S1, _: S2, _: S5, _: S6, _: S7) {}
        
        fn test14115() { foo14115(S0, S3, S4, S2, S0, S4, S1); }
    