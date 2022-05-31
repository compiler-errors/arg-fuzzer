
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4758(_: S1, _: S3, _: S6, _: S7) {}
        
        fn test4758() { foo4758(S3, S3, S2, S5, S6, S1, S3); }
    