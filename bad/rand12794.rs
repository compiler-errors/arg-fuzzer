
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12794(_: S1, _: S3, _: S4, _: S5, _: S6) {}
        
        fn test12794() { foo12794(S7, S4, S0, S1, S3, S7); }
    