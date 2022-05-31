
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9210(_: S1, _: S2, _: S3, _: S4, _: S6, _: S8) {}
        
        fn test9210() { foo9210(S2, S6, S3, S5, S0, S4, S1); }
    