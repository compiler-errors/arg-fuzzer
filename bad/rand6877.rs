
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6877(_: S2, _: S3, _: S4, _: S5, _: S7, _: S8) {}
        
        fn test6877() { foo6877(S7, S1, S5, S4, S5, S3, S1); }
    