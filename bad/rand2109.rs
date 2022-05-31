
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2109(_: S1, _: S3, _: S8) {}
        
        fn test2109() { foo2109(S1, S2, S3, S4, S5, S6); }
    