
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2768(_: S4, _: S5, _: S6, _: S7) {}
        
        fn test2768() { foo2768(S1, S1, S4, S6, S5, S3, S2); }
    