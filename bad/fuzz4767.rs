
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4767(_: S8, _: S1, _: S3, _: S4) {}
        
        fn test4767() { foo4767(S1, S1, S7, S6, S8, S2, S3, S7); }
    