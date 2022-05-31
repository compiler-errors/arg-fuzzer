
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1276(_: S3, _: S4) {}
        
        fn test1276() { foo1276(S5, S6, S8, S3, S2, S8, S1, S2); }
    