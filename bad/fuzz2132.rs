
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2132(_: S2, _: S3, _: S4, _: S6, _: S7, _: S8) {}
        
        fn test2132() { foo2132(S6, S5, S3, S6, S7, S6, S5); }
    