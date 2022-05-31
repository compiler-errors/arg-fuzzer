
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12784(_: S3, _: S5, _: S6, _: S8) {}
        
        fn test12784() { foo12784(S0, S6, S3, S5, S4); }
    