
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12572(_: S3, _: S7, _: S7) {}
        
        fn test12572() { foo12572(S6, S5, S0, S3, S0, S3); }
    