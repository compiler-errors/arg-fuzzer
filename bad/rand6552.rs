
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6552(_: S5, _: S1, _: S0) {}
        
        fn test6552() { foo6552(S3, S6, S6, S5, S0, S2); }
    