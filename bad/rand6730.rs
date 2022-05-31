
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6730(_: S1, _: S2, _: S3, _: S7, _: S8) {}
        
        fn test6730() { foo6730(S1, S6, S2, S4, S1, S0); }
    