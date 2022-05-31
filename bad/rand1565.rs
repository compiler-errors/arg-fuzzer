
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1565(_: S1, _: S7, _: S5) {}
        
        fn test1565() { foo1565(S1, S8, S2, S4, S7, S3); }
    