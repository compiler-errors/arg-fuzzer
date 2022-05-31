
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2679(_: S4, _: S3) {}
        
        fn test2679() { foo2679(S2, S4, S6, S7, S8); }
    