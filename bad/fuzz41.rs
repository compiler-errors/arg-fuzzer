
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo41(_: S2, _: S1, _: S3) {}
        
        fn test41() { foo41(S1, S4, S6, S2, S8, S5); }
    