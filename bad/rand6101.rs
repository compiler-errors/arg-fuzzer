
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6101(_: S4, _: S5, _: S7, _: S8) {}
        
        fn test6101() { foo6101(S5, S2, S6, S3, S4, S1); }
    