
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8936(_: S3, _: S5, _: S8) {}
        
        fn test8936() { foo8936(S0, S4, S5, S0); }
    