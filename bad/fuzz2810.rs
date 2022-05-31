
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2810(_: S5, _: S4, _: S8, _: S2, _: S7, _: S6, _: S3) {}
        
        fn test2810() { foo2810(S2, S7, S6, S8, S1, S6, S5, S5); }
    