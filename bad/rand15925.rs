
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15925(_: S1, _: S3, _: S8) {}
        
        fn test15925() { foo15925(S4, S1, S6, S2, S4, S4, S0); }
    