
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo67(_: S6, _: S4, _: S3, _: S1, _: S7, _: S6) {}
        
        fn test67() { foo67(S7, S4, S4, S6); }
    