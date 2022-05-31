
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2284(_: S7, _: S5, _: S4) {}
        
        fn test2284() { foo2284(S2, S3, S4, S6, S7, S8); }
    