
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2253(_: S7, _: S3, _: S5) {}
        
        fn test2253() { foo2253(S1, S1, S6, S7, S1); }
    