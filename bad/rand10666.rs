
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10666(_: S7, _: S6, _: S3, _: S8) {}
        
        fn test10666() { foo10666(S0, S2, S1, S3, S1); }
    