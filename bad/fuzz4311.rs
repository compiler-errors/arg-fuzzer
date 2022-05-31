
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4311(_: S2, _: S3, _: S5, _: S7) {}
        
        fn test4311() { foo4311(S8, S8, S3, S8, S8, S8); }
    