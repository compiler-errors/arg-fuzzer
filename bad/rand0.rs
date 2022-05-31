
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo0(_: S1, _: S5, _: S4, _: S8, _: S3) {}
        
        fn test0() { foo0(S3, S6, S7, S7, S3, S2, S4, S2); }
    