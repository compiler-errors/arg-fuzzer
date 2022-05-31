
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2220(_: S5, _: S6) {}
        
        fn test2220() { foo2220(S2, S3, S4, S5, S7, S8); }
    