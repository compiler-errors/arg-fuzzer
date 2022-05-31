
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2717(_: S5, _: S6, _: S7) {}
        
        fn test2717() { foo2717(S1, S7, S7, S5, S3, S5); }
    