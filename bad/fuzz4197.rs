
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4197(_: S1, _: S2, _: S3, _: S4, _: S5, _: S6, _: S7) {}
        
        fn test4197() { foo4197(S4, S8, S3, S1, S1, S6, S5, S4); }
    