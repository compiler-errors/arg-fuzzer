
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4380(_: S3, _: S5, _: S7) {}
        
        fn test4380() { foo4380(S1, S2, S3, S4, S6, S7, S8); }
    