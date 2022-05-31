
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5327(_: S1, _: S5, _: S8) {}
        
        fn test5327() { foo5327(S3, S6, S1, S7, S2); }
    