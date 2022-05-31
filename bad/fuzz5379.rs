
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5379(_: S2, _: S3, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test5379() { foo5379(S8, S3, S1, S6, S4, S5, S6, S2); }
    