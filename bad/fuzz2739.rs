
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2739(_: S4, _: S2, _: S1, _: S6, _: S8) {}
        
        fn test2739() { foo2739(S5, S4, S4, S1, S5, S3, S2, S2); }
    