
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1247(_: S3, _: S2, _: S7) {}
        
        fn test1247() { foo1247(S1, S2, S3, S4, S5, S6, S8); }
    