
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4925(_: S5, _: S0, _: S2) {}
        
        fn test4925() { foo4925(S1, S3, S4, S5, S6, S7); }
    