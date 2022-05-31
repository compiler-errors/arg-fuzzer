
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4965(_: S1, _: S3, _: S7) {}
        
        fn test4965() { foo4965(S8, S1, S2, S5); }
    