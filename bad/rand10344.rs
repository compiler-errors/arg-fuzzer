
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10344(_: S5, _: S7, _: S4, _: S6, _: S1, _: S1) {}
        
        fn test10344() { foo10344(S3, S5, S0, S0, S2, S3, S3); }
    