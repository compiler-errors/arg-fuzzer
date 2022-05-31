
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12292(_: S1, _: S4, _: S5) {}
        
        fn test12292() { foo12292(S2, S4, S1, S7, S0, S1); }
    