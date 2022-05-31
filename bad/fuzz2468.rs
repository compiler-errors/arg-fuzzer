
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2468(_: S2, _: S3, _: S4, _: S5, _: S8) {}
        
        fn test2468() { foo2468(S8, S5, S5, S3, S8, S2, S6, S3); }
    