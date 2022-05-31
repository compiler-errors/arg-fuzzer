
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2552(_: S6, _: S7, _: S4, _: S2, _: S4, _: S4) {}
        
        fn test2552() { foo2552(S1, S6, S5, S7, S3, S2, S8); }
    