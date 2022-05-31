
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1001(_: S3, _: S2) {}
        
        fn test1001() { foo1001(S8, S1, S5, S3, S4); }
    