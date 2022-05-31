
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13036(_: S4, _: S7, _: S5) {}
        
        fn test13036() { foo13036(S1, S5, S1, S0, S5, S4); }
    