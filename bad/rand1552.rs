
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1552(_: S3, _: S6, _: S8) {}
        
        fn test1552() { foo1552(S5, S4, S6, S3, S7, S6, S6); }
    