
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1358(_: S3, _: S4, _: S7) {}
        
        fn test1358() { foo1358(S1, S4, S5, S6, S5, S3, S6, S4); }
    